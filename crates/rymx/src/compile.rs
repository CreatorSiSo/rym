mod lir;
mod ty;

use codegen::settings::Flags;
use cranelift::codegen;
use cranelift::prelude::{isa, *};
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use lir::TypedExpr;
use std::sync::Arc;

mod symbol;

struct Session {
    builder_context: FunctionBuilderContext,
    context: codegen::Context,
    data_description: DataDescription,
    module: ObjectModule,
}

impl Session {
    pub fn new(target: &Target) -> Self {
        let module = ObjectModule::new(
            ObjectBuilder::new(
                target.isa(),
                "unnamed",
                cranelift_module::default_libcall_names(),
            )
            .unwrap(),
        );

        Self {
            builder_context: FunctionBuilderContext::new(),
            context: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    pub fn finish(self) -> ObjectProduct {
        self.module.finish()
    }

    pub fn compile_function(&mut self, name: &str, func: &ty::Function, body: &TypedExpr) {
        self.translate_function(func, body);

        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.context.func.signature)
            .unwrap();
        self.module.define_function(id, &mut self.context).unwrap();

        self.module.clear_context(&mut self.context);
    }

    fn translate_function(&mut self, func: &ty::Function, body: &TypedExpr) {
        let ptr_ty = self.module.target_config().pointer_type();
        let params = func.params.iter().map(|param| match abi_repr(param) {
            AbiRepr::Single(typ) => typ,
            AbiRepr::Pointer => ptr_ty,
        });

        let signature = &mut self.context.func.signature;
        {
            // Function params
            for param in params.clone() {
                signature.params.push(AbiParam::new(param));
            }
            // Function return
            signature
                .returns
                .push(AbiParam::new(match abi_repr(&func.result) {
                    AbiRepr::Single(typ) => typ,
                    AbiRepr::Pointer => ptr_ty,
                }));
        }

        let mut builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        for (i, typ) in params.enumerate() {
            let val = builder.block_params(entry_block)[i];
            let var = Variable::new(i);
            builder.declare_var(var, typ);
            builder.def_var(var, val);
        }

        // Now translate the statements of the function body.
        let mut trans = FunctionTranslator {
            builder,
            // variables,
            module: &mut self.module,
        };
        trans.translate_expr(body);

        let return_val = trans.builder.block_params(entry_block)[0];

        trans.builder.finalize();
    }
}

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    module: &'a mut ObjectModule,
}

impl FunctionTranslator<'_> {
    fn translate_expr(&mut self, expr: &TypedExpr) -> Value {
        match &expr.0 {
            lir::Expr::Literal(value) => self.translate_literal(*value, &expr.1),
            lir::Expr::Array(_) => todo!(),
            lir::Expr::Aggregate(_) => todo!(),
            lir::Expr::Unary(unary_op, typed_expr) => todo!(),
            lir::Expr::Binary(binary_op, l, r) => self.translate_binary(*binary_op, l, r),
            lir::Expr::Call(typed_expr, _) => todo!(),
            lir::Expr::AccessLocal(i) => self.builder.use_var(Variable::new(*i)),
            lir::Expr::AccessField(typed_expr, _) => todo!(),
            lir::Expr::Assign(typed_expr, typed_expr1) => todo!(),
            lir::Expr::Subscript(typed_expr, typed_expr1) => todo!(),
            lir::Expr::IfElse(typed_expr, typed_expr1, typed_expr2) => todo!(),
            lir::Expr::Loop(_) => todo!(),
            lir::Expr::Block(_) => todo!(),
            lir::Expr::Break(typed_expr) => todo!(),
            lir::Expr::Return(expr) => {
                let inner = self.translate_expr(expr);
                self.builder.ins().return_(&[inner]);
                // TODO This is extremely hacky!!
                Value::with_number(0).unwrap()
            }
        }
    }

    fn translate_literal(&mut self, value: u64, typ: &ty::Type) -> Value {
        let size = typ.layout().size().next_power_of_two();
        let int = Type::int_with_byte_size(size as u16).unwrap();
        self.builder.ins().iconst(int, value as i64)
    }

    fn translate_binary(&mut self, op: lir::BinaryOp, l: &TypedExpr, r: &TypedExpr) -> Value {
        if l.1 != r.1 {
            panic!(
                "Different types in binary expression, got ({:?}, {:?})!",
                l.1, r.1
            );
        }
        if !(matches!(l.1, ty::Type::Int(_) | ty::Type::Uint(_))
            || matches!(r.1, ty::Type::Int(_) | ty::Type::Uint(_)))
        {
            panic!(
                "Unsupported types in binary expression. Expected integers, got ({:?}, {:?})!",
                l.1, r.1
            );
        }

        let signed = matches!(l.1, ty::Type::Int(_));
        let l = self.translate_expr(l);
        let r = self.translate_expr(r);

        use lir::BinaryOp::*;
        let ins = self.builder.ins();
        match (op, signed) {
            (Add, _) => ins.iadd(l, r),
            (Sub, _) => ins.isub(l, r),
            (Mul, _) => ins.imul(l, r),
            (Div, _) => ins.udiv(l, r),
            (Eq, _) => ins.icmp(IntCC::Equal, l, r),
            (NotEq, _) => ins.icmp(IntCC::NotEqual, l, r),
            (LessThan, true) => ins.icmp(IntCC::SignedLessThan, l, r),
            (LessThanEq, true) => ins.icmp(IntCC::SignedLessThanOrEqual, l, r),
            (GreaterThan, true) => ins.icmp(IntCC::SignedGreaterThan, l, r),
            (GreaterThanEq, true) => ins.icmp(IntCC::SignedGreaterThanOrEqual, l, r),
            (LessThan, false) => ins.icmp(IntCC::UnsignedLessThan, l, r),
            (LessThanEq, false) => ins.icmp(IntCC::UnsignedLessThanOrEqual, l, r),
            (GreaterThan, false) => ins.icmp(IntCC::UnsignedGreaterThan, l, r),
            (GreaterThanEq, false) => ins.icmp(IntCC::UnsignedGreaterThanOrEqual, l, r),
        }
    }
}

enum AbiRepr {
    Single(Type),
    Pointer,
}

fn abi_repr(typ: &ty::Type) -> AbiRepr {
    let size = typ.layout().size();
    if size <= 1 {
        AbiRepr::Single(types::I8)
    } else if size <= 2 {
        AbiRepr::Single(types::I16)
    } else if size <= 4 {
        AbiRepr::Single(types::I32)
    } else if size <= 8 {
        AbiRepr::Single(types::I64)
    } else {
        AbiRepr::Pointer
    }
}

struct Target {
    isa: Arc<dyn isa::TargetIsa>,
}

impl Target {
    fn new(triple: &str) -> Self {
        let shared_builder = codegen::settings::builder();
        let shared_flags = Flags::new(shared_builder);
        let triple = target_lexicon::triple!(triple);
        let builder = isa::lookup(triple).expect("Could not obtain triple to construct target");
        let isa = builder
            .finish(shared_flags)
            .expect("Could not create ISA for target");

        Self { isa }
    }

    fn isa(&self) -> Arc<dyn isa::TargetIsa> {
        self.isa.clone()
    }
}

#[test]
fn function() {
    use lir::{BinaryOp, Expr, TypedExpr};
    use std::io::Write;
    use ty::{Function, Type};
    let ty_u8 = Type::Uint(8);

    let access_0 = TypedExpr(&Expr::AccessLocal(0), ty_u8);
    let access_1 = TypedExpr(&Expr::AccessLocal(1), ty_u8);
    let add = Expr::Binary(BinaryOp::Add, access_0, access_1);
    let sub = Expr::Binary(
        BinaryOp::Sub,
        TypedExpr(&add, ty_u8),
        TypedExpr(&Expr::Literal(10), ty_u8),
    );
    let ret = Expr::Return(TypedExpr(&sub, Type::Unit));

    let func = Function {
        params: &[ty_u8, ty_u8],
        result: ty_u8,
    };
    let body = TypedExpr(&ret, Type::Never);

    let mut session = Session::new(&Target::new("riscv64"));
    session.compile_function("add", &func, &body);
    let mut output = Vec::new();
    session.finish().object.emit(&mut output).unwrap();
    let mut file = std::fs::File::create("./out.o").unwrap();
    file.write_all(&output).unwrap();
}
