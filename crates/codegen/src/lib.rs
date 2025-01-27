mod lir;
mod test;
mod ty;

use codegen::settings::Flags;
use cranelift::codegen;
use cranelift::prelude::{isa, *};
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use lir::TypedExpr;
use std::sync::Arc;

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

        trans.builder.finalize();
    }
}

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    module: &'a mut ObjectModule,
}

impl FunctionTranslator<'_> {
    fn translate_expr(&mut self, expr: &TypedExpr) -> Option<Value> {
        match &expr.0 {
            lir::Expr::Literal(value) => Some(self.translate_literal(*value, &expr.1)),
            lir::Expr::Array(_) => todo!(),
            lir::Expr::Aggregate(_) => todo!(),
            lir::Expr::Unary(unary_op, typed_expr) => todo!(),
            lir::Expr::Binary(binary_op, l, r) => Some(self.translate_binary(*binary_op, l, r)),
            lir::Expr::Call(_, _) => todo!(),
            lir::Expr::AccessLocal(i) => Some(self.builder.use_var(Variable::new(*i))),
            lir::Expr::AccessField(_, _) => todo!(),
            lir::Expr::Assign(_, _) => todo!(),
            lir::Expr::Subscript(_, _) => todo!(),
            lir::Expr::IfElse(_, _, _) => todo!(),
            lir::Expr::Loop(body) => {
                self.translate_loop(body);
                None
            }
            lir::Expr::Block(_) => todo!(),
            lir::Expr::Break(_) => todo!(),
            lir::Expr::Return(expr) => {
                let inner = self.translate_expr(expr).unwrap();
                self.builder.ins().return_(&[inner]);
                None
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
        let l = self.translate_expr(l).unwrap();
        let r = self.translate_expr(r).unwrap();

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

    fn translate_loop(&mut self, body: &[lir::TypedExpr]) {
        let ty_u32 = Type::int(32).unwrap();

        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        // let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);
        self.builder.switch_to_block(header_block);

        self.builder.ins().jump(body_block, &[]);

        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        for expr in body {
            self.translate_expr(expr);
        }

        self.builder.ins().jump(header_block, &[]);

        // We've reached the bottom of the loop, so there will be no
        // more jumps to the header
        self.builder.seal_block(header_block);

        // self.builder.switch_to_block(exit_block);
        // self.builder.seal_block(exit_block);

        // const UNREACHABLE: u16 = 100;
        // self.builder.ins().trap(TrapCode::User(UNREACHABLE));
    }
}

pub fn generate_mark_custom<'a>(
    target: &Target,
    types: impl Iterator<Item = (u32, &'a ty::Type<'a>)>,
) -> ObjectModule {
    let Session {
        mut builder_context,
        mut context,
        mut module,
        ..
    } = Session::new(target);

    let ty_ptr = module.target_config().pointer_type();
    let params = [
        ty_ptr, // header pointer
        ty_ptr, // data pointer
    ];

    let signature = &mut context.func.signature;
    for typ in params {
        signature.params.push(AbiParam::new(typ));
    }

    let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

    let entry_block = builder.create_block();
    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    for (i, typ) in params.into_iter().enumerate() {
        let val = builder.block_params(entry_block)[i];
        let var = Variable::new(i);
        builder.declare_var(var, typ);
        builder.def_var(var, val);
    }

    let default = builder.create_block();
    builder.switch_to_block(default);
    builder.seal_block(default);
    builder.ins().trap(TrapCode::User(123));
    let default = builder.func.dfg.block_call(default, &[]);

    // for (typ_id, typ) in types {
    //     // TODO Generate body
    // }

    builder.create_jump_table(JumpTableData::new(default, &[]));

    // builder.ins().return_(&[]);

    builder.finalize();

    let id = module
        .declare_function("mark_custom", Linkage::Export, &context.func.signature)
        .unwrap();
    module.define_function(id, &mut context).unwrap();

    module
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

pub struct Target {
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
