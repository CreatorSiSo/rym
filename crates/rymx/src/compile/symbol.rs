use crate::ast::{Expr, Function};
use std::collections::HashMap;

/// Inspired by: https://github.com/RustPython/RustPython/blob/main/compiler/codegen/src/symboltable.rs

#[derive(Debug, Clone)]
pub struct SymbolTable {
	/// The name of this symbol table.
	/// Often the name of the module, struct or function.
	pub name: String,

	/// The type of symbol table
	pub typ: SymbolTableType,

	/// A set of symbols present in this scope
	pub symbols: HashMap<String, Symbol>,

	/// A list of sub-scopes in the order as found in the Ast.
	pub sub_tables: Vec<SymbolTable>,
}

impl SymbolTable {
	pub fn new(name: String, typ: SymbolTableType) -> Self {
		Self {
			name,
			typ,
			symbols: HashMap::new(),
			sub_tables: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum SymbolTableType {
	Module,
	Function,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
	pub name: String,
}

struct SymbolTableBuilder {
	/// Top level symbol tables, ie. global modules
	tabels: Vec<SymbolTable>,
}

impl SymbolTableBuilder {
	pub fn new() -> Self {
		Self { tabels: vec![] }
	}

	pub fn enter_scope(&mut self, name: &str, typ: SymbolTableType) {
		self.tabels.push(SymbolTable::new(name.into(), typ));
	}

	/// Pop symbol table and add it to sub_tables Vec of parent table.
	pub fn leave_scope(&mut self) {
		let table = self.tabels.pop().unwrap();
		self.tabels.last_mut().unwrap().sub_tables.push(table);
	}
	pub fn scan_expressions(&mut self, exprs: &[Expr]) {
		for expr in exprs {
			self.scan_expression(expr);
		}
	}

	pub fn scan_expression(&mut self, expr: &Expr) {
		match expr {
			Expr::Ident(_) => {
				// TODO
			}
			// Expr::FieldAccess(lhs, rhs) => {
			// 	self.scan_expression(lhs);
			// 	self.scan_expression(rhs);
			// }
			Expr::Function(func) => self.scan_function(func),
			Expr::Unary(_op, rhs) => self.scan_expression(rhs),
			Expr::Binary(_op, lhs, rhs) => {
				self.scan_expression(lhs);
				self.scan_expression(rhs);
			}
			Expr::Call(lhs, args) => {
				self.scan_expression(lhs);
				self.scan_expressions(args);
			}
			Expr::IfElse(test, then, other) => {
				self.scan_expression(test);
				self.scan_expression(then);
				self.scan_expression(other);
			}
			// Expr::Block(stmts) => self.scan_statements(stmts),
			Expr::Break(rhs) => self.scan_expression(rhs),
			Expr::Return(rhs) => self.scan_expression(rhs),
			// Expr::Var(_, name, rhs) => {
			// 	self.scan_expression(rhs);
			// 	self.register_name(name);
			// }
			_ => (),
		}
	}

	pub fn scan_function(&mut self, func: &Function) {
		let Function {
			name, params, body, ..
		} = func;
		self.enter_scope(
			name
				.as_ref()
				.map(AsRef::as_ref)
				.unwrap_or("<anonymous function>"),
			SymbolTableType::Function,
		);

		for (name, _typ) in params {
			self.register_name(name);
		}
		self.scan_expression(body);

		self.leave_scope();
	}

	pub fn register_name(&mut self, name: &str) {
		let Some(overridden) = self
			.tabels
			.last_mut()
			.unwrap()
			.symbols
			.insert(name.into(), Symbol { name: name.into() })
		else {
			return;
		};

		todo!("{overridden:?}");
	}
}
