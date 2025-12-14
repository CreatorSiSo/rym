use ast::{
    ExprKind, FunctionParam, SpannedExpr, SpannedFunction, SpannedStmt, Type, Typed, TypedExpr,
    TypedFunction, TypedStmt,
};
use span::Span;
use std::{collections::HashMap, sync::mpsc::Sender};

use crate::error::{Diagnostic, Level};

pub struct TypeChecker {
    pub symbol_table: SymbolTable,
    pub emitter: Sender<Diagnostic>,
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

                if typ != &typed_expr.extra.typ {
                    Diagnostic::new(
                        Level::Error,
                        format!(
                            "Variable has type {typ} but value is of type {}",
                            typed_expr.extra.typ
                        ),
                    )
                    .with_child(
                        [expr.extra].as_slice(),
                        Level::Error,
                        format!("{}", typed_expr.extra.typ),
                    )
                    .emit(self.emitter.clone());
                }

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
                kind: ExprKind::Literal(literal.clone()),
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

                if typed_a != typed_b {
                    use ast::BinaryOp::*;
                    let message = match op {
                        Add => format!(
                            "cannot add `{}` to `{}`",
                            typed_a.extra.typ, typed_b.extra.typ
                        ),
                        Sub => format!(
                            "cannot subtract `{}` from `{}`",
                            typed_b.extra.typ, typed_a.extra.typ
                        ),
                        Mul => format!(
                            "cannot mutiply `{}` and `{}`",
                            typed_a.extra.typ, typed_b.extra.typ
                        ),
                        Div => format!(
                            "cannot divide `{}` by `{}`",
                            typed_a.extra.typ, typed_b.extra.typ
                        ),
                        Assign => format!(
                            "cannot assign `{}` to `{}`",
                            typed_b.extra.typ, typed_a.extra.typ
                        ),
                        _ => format!(
                            "cannot compare `{}` with `{}`",
                            typed_a.extra.typ, typed_b.extra.typ
                        ),
                    };

                    Diagnostic::new(Level::Error, message)
                        .with_child(
                            [expr.extra],
                            Level::Error,
                            format!(
                                "no implementation for `{} {op} {}`",
                                typed_a.extra.typ, typed_b.extra.typ
                            ),
                        )
                        .emit(self.emitter.clone());
                }

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
                        Diagnostic::new(
                            Level::Error,
                            format!(
                                "function expected {} parameters, got {} arguments",
                                params.len(),
                                args.len(),
                            ),
                        )
                        .with_child(
                            [typed_func.extra.span],
                            Level::Help,
                            format!("expected {} parameters", params.len()),
                        )
                        .with_child(
                            {
                                let start = typed_args
                                    .first()
                                    .map(|arg| arg.extra.span.start)
                                    .unwrap_or(func.extra.end);
                                [Span::new(
                                    start,
                                    typed_args
                                        .last()
                                        .map(|arg| arg.extra.span.end)
                                        .unwrap_or(start),
                                )
                                .with_id(func.extra.id)]
                            },
                            Level::Error,
                            format!("provided {} arguments", args.len()),
                        )
                        .emit(self.emitter.clone());
                    }
                    for (param, arg) in params.iter().zip(typed_args.iter()) {
                        if arg.extra.typ != param.1 {
                            Diagnostic::new(Level::Error, "Mismatched types")
                                .with_child(
                                    [func.extra],
                                    Level::Help,
                                    "arguments to this function are incorrect",
                                )
                                .with_child(
                                    [arg.extra.span],
                                    Level::Error,
                                    format!("expected `{}`, found `{}`", param.1, arg.extra.typ),
                                )
                                .emit(self.emitter.clone());
                        }
                    }
                } else {
                    Diagnostic::new(
                        Level::Error,
                        format!("{} is not callable", typed_func.extra.typ),
                    )
                    .with_child([func.extra], Level::Error, "not callable")
                    .emit(self.emitter.clone());
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
