mod env;
mod function;

use std::{
    cmp::PartialOrd,
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

pub use self::env::Env;
use self::env::ScopeKind;
pub use self::function::{Call, NativeFunction};
use crate::{
    ast::{BinaryOp, Expr, Function, Literal, Module, Stmt, UnaryOp, VariableKind},
    error::{Diagnostic, Level},
};

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Struct(HashMap<String, Value>),
    Function(Function),
    NativeFunction(NativeFunction),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            (Self::NativeFunction(l0), Self::NativeFunction(r0)) => l0 == r0,
            (Value::Unit, Value::Unit) => true,
            // TODO These cases should not be accessible, protect them via type checking
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool(inner) => f.write_str(if *inner { "true" } else { "false" }),
            Self::Int(inner) => write!(f, "{inner:#}"),
            Self::Float(inner) => write!(f, "{inner:#}"),
            Self::String(inner) => write!(f, "{inner:#}"),
            Self::Struct(inner) => write!(f, "<unkown> {inner:#?}"),
            Self::Function(inner) => write!(f, "{inner:#}"),
            Self::NativeFunction(inner) => write!(f, "{inner:#}"),
        }
    }
}

pub enum ControlFlow {
    /// Crashes the entire evaluation context
    Exit,
    None(Value),
    Break(Value),
    Return(Value),
}

macro_rules! default_flow {
    ($control_flow:expr) => {
        match $control_flow {
            ControlFlow::None(inner) => inner,
            control_flow => return control_flow,
        }
    };
}

pub trait Interpret {
    fn eval(self, env: &mut Env) -> ControlFlow;
}

impl Interpret for Module {
    fn eval(self, env: &mut Env) -> ControlFlow {
        // TODO sort based on dependency
        // self
        // 	.constants
        // 	.sort_by(|Constant { expr: l, .. }, Constant { expr: r, .. }| match (l, r) {});

        for (name, _, expr) in self.constants {
            // Top level, ignoring control flow
            let val = match expr.eval(env) {
                ControlFlow::None(inner)
                | ControlFlow::Break(inner)
                | ControlFlow::Return(inner) => inner,
                exit => return exit,
            };
            env.create(name, VariableKind::Const, val);
        }

        // TODO only do this when requested, ie. in main.rym file
        if let Some(main) = env.get("main") {
            match main {
                Value::Function(val) => {
                    // TODO avoid cloning here
                    val.clone().call(env, vec![]);
                }
                _ => todo!(),
            }
        }

        ControlFlow::Break(Value::Unit)
    }
}

impl Interpret for Stmt {
    fn eval(self, env: &mut Env) -> ControlFlow {
        match self {
            Stmt::Expr(expr) => expr.eval(env),
            Stmt::Variable(kind, name, typ, expr) => {
                let val = default_flow!(expr.eval(env));
                env.create(name, kind, val);
                ControlFlow::None(Value::Unit)
            }
            _ => todo!(),
        }
    }
}

impl Interpret for Expr {
    fn eval(self, env: &mut Env) -> ControlFlow {
        let result = match self {
            Expr::Unit => Value::Unit,
            Expr::Literal(lit) => match lit {
                Literal::Bool(inner) => Value::Bool(inner),
                Literal::Int(inner) => Value::Int(inner),
                Literal::Float(inner) => Value::Float(inner),
                Literal::String(inner) => Value::String(inner),
            },
            Expr::Array(array) => {
                todo!();
            }
            Expr::ArrayWithRepeat(value, length) => {
                todo!();
            }
            Expr::Struct(name, fields) => Value::Struct({
                let mut map = HashMap::with_capacity(fields.len());
                for (name, expr) in fields {
                    map.insert(name, default_flow!(expr.eval(env)));
                }
                map
            }),
            Expr::Function(func) => Value::Function(func),

            Expr::Unary(op, expr) => match (op, default_flow!(expr.eval(env))) {
                (UnaryOp::Neg, Value::Float(val)) => Value::Float(-val),
                (UnaryOp::Neg, Value::Int(val)) => Value::Int(-val),
                (UnaryOp::Not, Value::Bool(val)) => Value::Bool(!val),

                (_op, _val) => todo!(),
            },
            Expr::Binary(op, lhs, rhs) => match (
                op,
                default_flow!(lhs.eval(env)),
                default_flow!(rhs.eval(env)),
            ) {
                (op, Value::Float(lhs), Value::Float(rhs)) => {
                    eval_binary(op, lhs, rhs, Value::Float)
                }
                (op, Value::Float(lhs), Value::Int(rhs)) => {
                    let rhs = rhs as f64;
                    eval_binary(op, lhs, rhs, Value::Float)
                }
                (op, Value::Int(lhs), Value::Float(rhs)) => {
                    let lhs = lhs as f64;
                    eval_binary(op, lhs, rhs, Value::Float)
                }

                (op, Value::Int(lhs), Value::Int(rhs)) => eval_binary(op, lhs, rhs, Value::Int),

                (BinaryOp::Add, Value::String(lhs), Value::String(rhs)) => {
                    Value::String(lhs + &rhs)
                }

                (BinaryOp::Eq, lhs, rhs) => Value::Bool(lhs == rhs),
                (BinaryOp::NotEq, lhs, rhs) => Value::Bool(lhs != rhs),

                (_op, _lhs, _rhs) => todo!(),
            },
            Expr::Call(lhs, args) => {
                let result = match default_flow!(lhs.eval(env)) {
                    Value::Function(inner) => {
                        let mut arg_values = vec![];
                        for expr in args {
                            arg_values.push(default_flow!(expr.eval(env)));
                        }
                        inner.call(env, arg_values)
                    }
                    Value::NativeFunction(inner) => {
                        let mut arg_values = vec![];
                        for expr in args {
                            arg_values.push(default_flow!(expr.eval(env)));
                        }
                        inner.call(env, arg_values)
                    }
                    _ => todo!("Add error, value is not a function."),
                };
                match result {
                    ControlFlow::Exit => return ControlFlow::Exit,
                    ControlFlow::None(inner) => inner,
                    ControlFlow::Break(inner) => inner,
                    ControlFlow::Return(inner) => inner,
                }
            }

            Expr::Ident(name) => match env.get(&name) {
                // TODO Only clone when needed / faster
                Some(val) => val.clone(),
                None => {
                    Diagnostic::new(Level::Error, format!("Unable to find '{name}'"))
                        .emit(env.emitter.clone());
                    return ControlFlow::Exit;
                }
            },
            Expr::FieldAccess(lhs, key) => {
                let val = default_flow!(lhs.eval(env));
                let Some(field) = (match &val {
                    Value::Struct(fields) => fields.get(&key).cloned(),
                    _ => None,
                }) else {
                    Diagnostic::new(
                        Level::Error,
                        format!("Field '{key}' does not exist on value '{val}'"),
                    )
                    .emit(env.emitter.clone());
                    return ControlFlow::Exit;
                };
                field
            }
            Expr::Subscript(_lhs, _rhs) => {
                // TODO
                debug_todo();
                Value::Unit
            }

            Expr::IfElse(cond_expr, then_expr, else_expr) => {
                let Value::Bool(condition) = default_flow!(cond_expr.eval(env)) else {
                    todo!();
                };
                if condition {
                    default_flow!(then_expr.eval(env))
                } else {
                    default_flow!(else_expr.eval(env))
                }
            }
            Expr::Block(stmts) => {
                env.push_scope(ScopeKind::Expr);
                let mut result = Value::Unit;
                'stmts_loop: for stmt in stmts {
                    match stmt.eval(env) {
                        ControlFlow::None(_) => (),
                        ControlFlow::Break(inner) => {
                            result = inner;
                            break 'stmts_loop;
                        }
                        control_flow => return control_flow,
                    }
                }
                env.pop_scope();
                result
            }
            Expr::Break(expr) => return ControlFlow::Break(default_flow!(expr.eval(env))),
            Expr::Return(expr) => return ControlFlow::Return(default_flow!(expr.eval(env))),
        };

        ControlFlow::None(result)
    }
}

fn eval_binary<T>(op: BinaryOp, lhs: T, rhs: T, make_value: fn(T) -> Value) -> Value
where
    T: PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    match op {
        BinaryOp::Add => make_value(lhs + rhs),
        BinaryOp::Sub => make_value(lhs - rhs),
        BinaryOp::Mul => make_value(lhs * rhs),
        BinaryOp::Div => make_value(lhs / rhs),

        BinaryOp::LessThan => Value::Bool(lhs < rhs),
        BinaryOp::LessThanEq => Value::Bool(lhs <= rhs),
        BinaryOp::GreaterThan => Value::Bool(lhs > rhs),
        BinaryOp::GreaterThanEq => Value::Bool(lhs >= rhs),
        BinaryOp::Eq => Value::Bool(lhs == rhs),
        BinaryOp::NotEq => Value::Bool(lhs != rhs),

        BinaryOp::Assign => {
            debug_todo();
            Value::Unit
        }
    }
}

#[track_caller]
fn debug_todo() {
    println!("Hit TODO: {}", std::panic::Location::caller());
}
