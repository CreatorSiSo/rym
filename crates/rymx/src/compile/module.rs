use super::{bytecode::ByteCode, Constant, Type};
use std::collections::HashMap;

pub struct ModulePrototype {
    pub constants: HashMap<String, (Type, Constant)>,
    pub functions: HashMap<String, FunctionPrototype>,
    pub sub_modules: Vec<ModulePrototype>,
}

impl ModulePrototype {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
            functions: HashMap::new(),
            sub_modules: vec![],
        }
    }
}

pub struct FunctionPrototype {
    body: ByteCode,
}
