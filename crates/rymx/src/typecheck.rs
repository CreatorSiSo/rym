use ast::{
    ExprKind, FunctionParam, SpannedExpr, SpannedFunction, SpannedStmt, Type, Typed, TypedExpr,
    TypedFunction, TypedStmt,
};
use std::collections::HashMap;

pub struct TypeChecker {
    pub symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn typecheck_function(&mut self, func: &SpannedFunction) -> TypedFunction {
        self.symbol_table.push_scope();
        for param in &func.params {
            self.symbol_table
                .insert(param.name.clone(), param.typ.clone());
        }

        let typed_body = self.typecheck_expression(&func.body);
        assert_eq!(typed_body.extra.typ, func.return_type);

        let typed_params = func
            .params
            .iter()
            .map(|param| FunctionParam {
                name: param.name.clone(),
                typ: param.typ.clone(),
                default_value: param
                    .default_value
                    .clone()
                    .map(|expr| self.typecheck_expression(&expr)),
            })
            .collect();

        self.symbol_table.pop_scope();

        TypedFunction {
            name: func.name.clone(),
            params: typed_params,
            return_type: func.return_type.clone(),
            body: Box::new(typed_body),
        }
    }

    fn typecheck_statement(self: &mut TypeChecker, stmt: &SpannedStmt) -> TypedStmt {
        match stmt {
            SpannedStmt::Expr(expr) => TypedStmt::Expr(self.typecheck_expression(&*expr)),
            SpannedStmt::Function(func) => TypedStmt::Function(self.typecheck_function(func)),
            SpannedStmt::Variable(kind, name, typ, expr) => {
                self.symbol_table.insert(name.clone(), typ.clone());
                let typed_expr = self.typecheck_expression(expr);
                assert_eq!(typ, &typed_expr.extra.typ);
                TypedStmt::Variable(*kind, name.clone(), typ.clone(), typed_expr)
            }
        }
    }

    fn typecheck_expression(self: &mut TypeChecker, expr: &SpannedExpr) -> TypedExpr {
        match &expr.kind {
            ExprKind::Error => TypedExpr {
                kind: ExprKind::Error,
                extra: Typed::new(expr.extra, Type::Unkown),
            },
            ExprKind::Unit => TypedExpr {
                kind: ExprKind::Error,
                extra: Typed::new(expr.extra, Type::Unit),
            },
            ExprKind::Literal(literal) => TypedExpr {
                kind: ExprKind::Error,
                extra: Typed::new(expr.extra, type_of_literal(literal)),
            },
            ExprKind::Unary(op, expr) => {
                let typed_expr = self.typecheck_expression(expr);

                TypedExpr {
                    extra: Typed::new(expr.extra, typed_expr.extra.typ.clone()),
                    kind: ExprKind::Unary(*op, Box::new(typed_expr)),
                }
            }
            ExprKind::Binary(op, a, b) => {
                let typed_a = self.typecheck_expression(a);
                let typed_b = self.typecheck_expression(b);
                assert_eq!(typed_a, typed_b);

                TypedExpr {
                    extra: Typed::new(expr.extra, typed_a.extra.typ.clone()),
                    kind: ExprKind::Binary(*op, Box::new(typed_a), Box::new(typed_b)),
                }
            }
            ExprKind::Call(func, args) => {
                let typed_func = self.typecheck_expression(func);
                let typed_args: Vec<_> = args
                    .iter()
                    .map(|arg| self.typecheck_expression(arg))
                    .collect();
                let mut return_type = Type::Unkown;

                if let Type::Function { params, result } = &typed_func.extra.typ {
                    return_type = *result.clone();
                    if params.len() != args.len() {
                        eprintln!(
                            "Error: <{func:?}> expected {} parameters, got {} arguments",
                            params.len(),
                            args.len()
                        );
                    }
                    for (param, arg) in params.iter().zip(typed_args.iter()) {
                        assert_eq!(arg.extra.typ, param.1);
                    }
                } else {
                    eprintln!("Error: <{func:?}> is not callable!");
                };

                TypedExpr {
                    kind: ExprKind::Call(Box::new(typed_func), typed_args),
                    extra: Typed::new(expr.extra, return_type),
                }
            }
            ExprKind::Ident(ident) => {
                let typ = self.symbol_table.lookup(ident).unwrap_or_else(|| {
                    eprintln!("Error: Could not find definition for <{ident}>!");
                    Type::Unkown
                });

                TypedExpr {
                    kind: ExprKind::Ident(ident.clone()),
                    extra: Typed::new(expr.extra, typ),
                }
            }
            ExprKind::IfElse(cond, a, b) => {
                let typed_cond = self.typecheck_expression(cond);
                assert_eq!(typed_cond.extra.typ, Type::Bool);

                let typed_a = self.typecheck_expression(a);
                let typed_b = self.typecheck_expression(b);
                assert_eq!(typed_a, typed_b);

                TypedExpr {
                    extra: Typed::new(expr.extra, typed_a.extra.typ.clone()),
                    kind: ExprKind::IfElse(
                        Box::new(typed_cond),
                        Box::new(typed_a),
                        Box::new(typed_b),
                    ),
                }
            }
            ExprKind::Block(stmts) => {
                self.symbol_table.push_scope();
                let typed_stmts: Vec<_> = stmts
                    .iter()
                    .map(|stmt| self.typecheck_statement(stmt))
                    .collect();
                self.symbol_table.pop_scope();

                TypedExpr {
                    kind: ExprKind::Block(typed_stmts),
                    extra: Typed::new(expr.extra, /* TODO */ Type::Unit),
                }
            }
            ExprKind::Break(expr) => TypedExpr {
                kind: self.typecheck_expression(expr).kind,
                extra: Typed::new(expr.extra, Type::Never),
            },
            ExprKind::Return(expr) => TypedExpr {
                kind: self.typecheck_expression(expr).kind,
                extra: Typed::new(expr.extra, Type::Never),
            },
            ExprKind::Function(func) => TypedExpr {
                kind: ExprKind::Function(self.typecheck_function(func)),
                extra: Typed::new(expr.extra, type_of_function(func)),
            },
            _ => todo!(),
        }
    }

    // fn typecheck_params(params: &[FunctionParam<Span>]) -> Vec<FunctionParam<(Span, Type)>> {}
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
        self.scopes.last_mut().unwrap().insert(symbol, typ);
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
