use super::{Constant, Type};
use std::collections::HashMap;

pub struct TypedModule {
	pub constants: HashMap<String, (Type, Constant)>,
	pub functions: HashMap<String, TypedFunction>,
	pub sub_modules: Vec<TypedModule>,
}

impl TypedModule {
	pub fn new() -> Self {
		Self {
			constants: HashMap::new(),
			functions: HashMap::new(),
			sub_modules: vec![],
		}
	}
}

pub struct TypedFunction {}
