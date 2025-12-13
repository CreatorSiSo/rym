use ast::{SpannedExpr, SpannedFunction, SpannedStmt, Type};
use std::collections::HashMap;

pub struct TypeChecker {
    pub symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn typecheck_function(&mut self, func: &SpannedFunction) -> Type {
        self.symbol_table.push_scope();
        for param in &func.params {
            self.symbol_table
                .insert(param.name.clone(), param.typ.clone());
        }

        let type_result = self.typecheck_expression(&func.body);
        assert_eq!(type_result, func.return_type);

        self.symbol_table.pop_scope();

        Type::Function {
            params: func
                .params
                .iter()
                .map(|param| (param.name.clone(), param.typ.clone()))
                .collect(),
            result: Box::new(func.return_type.clone()),
        }
    }

    fn typecheck_statement(self: &mut TypeChecker, stmt: &SpannedStmt) -> Type {
        match stmt {
            ast::Stmt::Expr(expr) => self.typecheck_expression(&*expr),
            ast::Stmt::Function(func) => self.typecheck_function(func),
            ast::Stmt::Variable(_, name, typ, expr) => {
                self.symbol_table.insert(name.clone(), typ.clone());
                assert_eq!(typ, &self.typecheck_expression(expr));
                Type::Unit
            }
        }
    }

    fn typecheck_expression(self: &mut TypeChecker, expr: &SpannedExpr) -> Type {
        match &expr.kind {
            ast::ExprKind::Error => Type::Unkown,
            ast::ExprKind::Unit => Type::Unit,
            ast::ExprKind::Literal(literal) => type_of_literal(literal),
            ast::ExprKind::Unary(_, expr) => self.typecheck_expression(expr),
            ast::ExprKind::Binary(_, a, b) => {
                let type_a = self.typecheck_expression(a);
                let type_b = self.typecheck_expression(b);
                assert_eq!(type_a, type_b);
                type_a
            }
            ast::ExprKind::Call(func, args) => {
                let Type::Function { params, result } = self.typecheck_expression(func) else {
                    eprintln!("Error: <{func:?}> is not callable!");
                    return Type::Unkown;
                };

                if params.len() != args.len() {
                    eprintln!(
                        "Error: <{func:?}> expected {} parameters, got {} arguments",
                        params.len(),
                        args.len()
                    );
                }
                for (param, arg) in params.iter().zip(args) {
                    let type_arg = self.typecheck_expression(arg);
                    assert_eq!(type_arg, param.1);
                }
                *result
            }
            ast::ExprKind::Ident(ident) => {
                let Some(typ) = self.symbol_table.lookup(ident) else {
                    eprintln!("Error: Could not find definition for <{ident}>!");
                    return Type::Unkown;
                };
                typ
            }
            ast::ExprKind::IfElse(cond, a, b) => {
                let type_cond = self.typecheck_expression(cond);
                assert_eq!(type_cond, Type::Bool);

                let type_a = self.typecheck_expression(a);
                let type_b = self.typecheck_expression(b);
                assert_eq!(type_a, type_b);

                type_a
            }
            ast::ExprKind::Block(stmts) => {
                for stmt in stmts {
                    self.typecheck_statement(stmt);
                }
                // TODO
                Type::Unit
            }
            ast::ExprKind::Break(expr) => {
                self.typecheck_expression(expr);
                Type::Never
            }
            ast::ExprKind::Return(expr) => {
                self.typecheck_expression(expr);
                Type::Never
            }
            ast::ExprKind::Function(func) => type_of_function(func),
            _ => todo!(),
        }
    }
}

pub fn type_of_function(func: &SpannedFunction) -> Type {
    Type::Function {
        params: func
            .params
            .iter()
            .map(|param| (param.name.clone(), param.typ.clone()))
            .collect(),
        result: Box::new(func.return_type.clone()),
    }
}

pub fn type_of_literal(literal: &ast::Literal) -> Type {
    match literal {
        ast::Literal::Int(_) => Type::Int,
        ast::Literal::Float(_) => Type::Float,
        ast::Literal::String(_) => Type::String,
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Type>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut result = Self { scopes: Vec::new() };
        result.push_scope();
        result
    }

    pub fn lookup(&self, symbol: impl AsRef<str>) -> Option<Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(symbol.as_ref()).cloned())
    }

    pub fn insert(&mut self, symbol: String, typ: Type) {
        println!("{symbol} : {typ}");
        self.scopes.last_mut().unwrap().insert(symbol, typ);
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
