use std::collections::HashMap;

use ast::Type;

pub struct Context {
    pub symbol_table: SymbolTable,
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

pub fn typecheck_function(ctx: &mut Context, func: &ast::Function) -> Type {
    ctx.symbol_table.push_scope();
    for param in &func.params {
        ctx.symbol_table
            .insert(param.name.clone(), param.typ.clone());
    }

    let type_result = typecheck_expression(ctx, &*func.body);
    assert_eq!(type_result, func.return_type);

    ctx.symbol_table.pop_scope();

    Type::Function {
        params: func
            .params
            .iter()
            .map(|param| (param.name.clone(), param.typ.clone()))
            .collect(),
        result: Box::new(func.return_type.clone()),
    }
}

fn typecheck_statement(ctx: &mut Context, stmt: &ast::Stmt) -> Type {
    match stmt {
        ast::Stmt::Expr(expr) => typecheck_expression(ctx, expr),
        ast::Stmt::Function(func) => typecheck_function(ctx, func),
        ast::Stmt::Variable(_, name, typ, expr) => {
            ctx.symbol_table.insert(name.clone(), typ.clone());
            assert_eq!(typ, &typecheck_expression(ctx, expr));
            Type::Unit
        }
    }
}

fn typecheck_expression(ctx: &mut Context, expr: &ast::Expr) -> Type {
    match expr {
        ast::Expr::Error => Type::Unkown,
        ast::Expr::Unit => Type::Unit,
        ast::Expr::Literal(literal) => typecheck_literal(literal),
        ast::Expr::Unary(_, expr) => typecheck_expression(ctx, expr),
        ast::Expr::Binary(_, a, b) => {
            let type_a = typecheck_expression(ctx, a);
            let type_b = typecheck_expression(ctx, b);
            assert_eq!(type_a, type_b);
            type_a
        }
        ast::Expr::Call(func, args) => {
            let Type::Function { params, result } = typecheck_expression(ctx, func) else {
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
                let type_arg = typecheck_expression(ctx, arg);
                assert_eq!(type_arg, param.1);
            }
            *result
        }
        ast::Expr::Ident(ident) => {
            let Some(typ) = ctx.symbol_table.lookup(ident) else {
                eprintln!("Error: Could not find definition for <{ident}>!");
                return Type::Unkown;
            };
            typ
        }
        ast::Expr::IfElse(cond, a, b) => {
            let type_cond = typecheck_expression(ctx, cond);
            assert_eq!(type_cond, Type::Bool);

            let type_a = typecheck_expression(ctx, a);
            let type_b = typecheck_expression(ctx, b);
            assert_eq!(type_a, type_b);

            type_a
        }
        ast::Expr::Block(stmts) => {
            for stmt in stmts {
                typecheck_statement(ctx, stmt);
            }
            // TODO
            Type::Unit
        }
        ast::Expr::Break(expr) => {
            typecheck_expression(ctx, expr);
            Type::Never
        }
        ast::Expr::Return(expr) => {
            typecheck_expression(ctx, expr);
            Type::Never
        }
        _ => todo!(),
    }
}

fn typecheck_literal(literal: &ast::Literal) -> Type {
    match literal {
        ast::Literal::Int(_) => Type::Int,
        ast::Literal::Float(_) => Type::Float,
        ast::Literal::String(_) => Type::String,
    }
}

pub struct TypecheckResult {
    value: Type,
    returns: Option<Type>,
}
