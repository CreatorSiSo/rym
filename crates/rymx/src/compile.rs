mod ir;
use ir as rym_ir;
mod ty;
use ty as rym_ty;

use codegen::settings::Flags;
use cranelift::codegen;
use cranelift::prelude::{isa, *};
use cranelift_module::{DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
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

    pub fn compile_function(&mut self, name: &str, ast: &rym_ir::Function) {
        self.translate_function(ast);

        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.context.func.signature)
            .unwrap();
        self.module.define_function(id, &mut self.context).unwrap();

        self.module.clear_context(&mut self.context);
    }

    fn translate_function(&mut self, function: &rym_ir::Function) {
        let ptr_ty = self.module.target_config().pointer_type();
        let params = function.params.iter().map(|param| match abi_repr(param) {
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
                .push(AbiParam::new(match abi_repr(&function.result) {
                    AbiRepr::Single(typ) => typ,
                    AbiRepr::Pointer => ptr_ty,
                }));
        }

        let mut builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // TODO Translate statements

        let return_val = builder.block_params(entry_block)[0];
        builder.ins().return_(&[return_val]);

        builder.finalize();
    }
}

enum AbiRepr {
    Single(Type),
    Pointer,
}

fn abi_repr(typ: &rym_ty::Type) -> AbiRepr {
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
    use rym_ir::{BinaryOp, Expr, Function};
    use rym_ty::Type;
    use std::io::Write;

    let func = Function {
        params: &[Type::Uint(8), Type::Uint(8)],
        result: Type::Uint(8),
        body: &Expr::Return(&Expr::Binary(BinaryOp::Add, &Expr::Load(0), &Expr::Load(1))),
    };

    let mut session = Session::new(&Target::new("riscv64"));
    session.compile_function("simple", &func);
    let mut output = Vec::new();
    session.finish().object.emit(&mut output).unwrap();
    let mut file = std::fs::File::create("./out.o").unwrap();
    file.write_all(&output).unwrap();
}
