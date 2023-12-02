## Match

### Ideas

```rym
const fib = fn(n) match n with
    | 0 => 0,
    | 1 => 1,
    | _ => fib(n - 1) + fib(n - 2);

const fib = fn(n) match n:
    | 0 => 0,
    | 1 => 1,
    | _ => fib(n - 1) + fib(n - 2);

const fib = fn(n) match n {
    0 => 0,
    1 => 1,
    _ => fib(n - 1) + fib(n - 2),
};
```

```rym
const fib = fn(n) match n with
    | (0 || 1) @ n0 => match n0 with
        | 0 => 1,
        | 1 => 0,
    | _ => fib(n - 1) + fib(n - 2);

const fib = fn(n) match n:
    | (0 || 1) @ n0 => match n0:
        | 0 => 1,
        | 1 => 0,
    | _ => fib(n - 1) + fib(n - 2);

const fib = fn(n) match n {
    (0 || 1) @ n0 => match n0 {
        0 => 1,
        1 => 0,
    },
    _ => fib(n - 1) + fib(n - 2),
};
```

### Complex

```rym
match expr with
    | Expr.Unit => Value.Unit,
    | Expr.Literal(lit) => match lit with
        | Literal.Bool(inner) => Value.Bool(inner),
        | Literal.Int(inner) => Value.Int(inner),
        | Literal.Float(inner) => Value.Float(inner),
        \ Literal.String(inner) => Value.String(inner),
    | Expr.Ident(name) => {
        // TODO Only clone when needed / faster
        env.get(name).unwrap().clone()
    },
    | Expr.Function(f) => Value.Function(f),

    | Expr.Unary(op, expr) => match (op, todo(expr.eval(env))) with
        | (UnaryOp.Neg, Value.Float(val)) => Value.Float(-val),
        | (UnaryOp.Neg, Value.Int(val)) => Value.Int(-val),
        | (UnaryOp.Not, Value.Bool(val)) => Value.Bool(!val),
        \ (op, val) => todo(),
    | Expr.Binary(op, lhs, rhs) => match (
        op,
        todo(lhs.eval(env)),
        todo(rhs.eval(env)),
    ) with
        | (BinaryOp.Add, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs + rhs),
        | (BinaryOp.Add, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) + rhs),
        | (BinaryOp.Add, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs + cast(rhs, f64)),
        | (BinaryOp.Sub, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs - rhs),
        | (BinaryOp.Sub, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) - rhs),
        | (BinaryOp.Sub, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs - cast(rhs, f64)),
        | (BinaryOp.Mul, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs * rhs),
        | (BinaryOp.Mul, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) * rhs),
        | (BinaryOp.Mul, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs * cast(rhs, f64)),
        | (BinaryOp.Div, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs / rhs),
        | (BinaryOp.Div, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) / rhs),
        | (BinaryOp.Div, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs / cast(rhs, f64)),

        | (BinaryOp.Add, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs + rhs),
        | (BinaryOp.Sub, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs - rhs),
        | (BinaryOp.Mul, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs * rhs),
        | (BinaryOp.Div, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs / rhs),

        | (BinaryOp.Add, Value.String(lhs), Value.String(rhs)) => Value.String(lhs + rhs),

        | (BinaryOp.Eq, lhs, rhs) => Value.Bool(lhs == rhs),
        | (BinaryOp.NotEq, lhs, rhs) => Value.Bool(lhs != rhs),

        \ (op, lhs, rhs) => todo(),
    | Expr.Call(lhs, args) => match todo(lhs.eval(env)) with
        | Value.Function(inner) => {
            let mut arg_values = Vec.new();
            for expr in args {
                arg_values.push(todo(expr.eval(env)));
            }
            inner.call(env, arg_values)
        }
        | Value.NativeFunction(inner) => {
            let mut arg_values = Vec.new();
            for expr in args {
                arg_values.push(todo(expr.eval(env)));
            }
            inner.call(env, arg_values)
        }
        \ _ => todo("Add error, value is not a function."),

    | Expr.IfElse(cond_expr, then_expr, else_expr) => {
        let Value.Bool(condition) = todo(cond_expr.eval(env)) else {
            todo();
        };
        if condition then
            todo(then_expr.eval(env))
        else
            todo(else_expr.eval(env))
    }
    | Expr.Block(exprs) => {
        env.push_scope(ScopeKind.Expr);
        let mut result = Value.Unit;
        #exprs_loop: for expr in exprs {
            match expr.eval(env) {
                ControlFlow.None(_) => (),
                ControlFlow.Break(inner) => {
                    result = inner;
                    break #exprs_loop;
                }
                control_flow => return control_flow,
            }
        }
        env.pop_scope();
        result
    }
    | Expr.Break(expr) => return ControlFlow.Break(todo(expr.eval(env))),
    | Expr.Return(expr) => return ControlFlow.Return(todo(expr.eval(env))),

    \ Expr.Var(kind, name, expr) => {
        let val = todo(expr.eval(env));
        env.create(name, kind, val);
        Value.Unit
    }

match expr {
    Expr.Unit => Value.Unit,
    Expr.Literal(lit) => match lit {
        Literal.Bool(inner) => Value.Bool(inner),
        Literal.Int(inner) => Value.Int(inner),
        Literal.Float(inner) => Value.Float(inner),
        Literal.String(inner) => Value.String(inner),
    },
    Expr.Ident(name) => {
        // TODO Only clone when needed / faster
        env.get(name).unwrap().clone()
    }
    Expr.Function(func) => Value.Function(func),

    Expr.Unary(op, expr) => match (op, todo(expr.eval(env))) {
        (UnaryOp.Neg, Value.Float(val)) => Value.Float(-val),
        (UnaryOp.Neg, Value.Int(val)) => Value.Int(-val),
        (UnaryOp.Not, Value.Bool(val)) => Value.Bool(!val),
        (op, val) => todo(),
    },
    Expr.Binary(op, lhs, rhs) => match (
        op,
        todo(lhs.eval(env)),
        todo(rhs.eval(env)),
    ) {
        (BinaryOp.Add, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs + rhs),
        (BinaryOp.Add, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) + rhs),
        (BinaryOp.Add, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs + cast(rhs, f64)),
        (BinaryOp.Sub, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs - rhs),
        (BinaryOp.Sub, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) - rhs),
        (BinaryOp.Sub, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs - cast(rhs, f64)),
        (BinaryOp.Mul, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs * rhs),
        (BinaryOp.Mul, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) * rhs),
        (BinaryOp.Mul, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs * cast(rhs, f64)),
        (BinaryOp.Div, Value.Float(lhs), Value.Float(rhs)) => Value.Float(lhs / rhs),
        (BinaryOp.Div, Value.Int(lhs), Value.Float(rhs)) => Value.Float(cast(lhs, f64) / rhs),
        (BinaryOp.Div, Value.Float(lhs), Value.Int(rhs)) => Value.Float(lhs / cast(rhs, f64)),

        (BinaryOp.Add, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs + rhs),
        (BinaryOp.Sub, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs - rhs),
        (BinaryOp.Mul, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs * rhs),
        (BinaryOp.Div, Value.Int(lhs), Value.Int(rhs)) => Value.Int(lhs / rhs),

        (BinaryOp.Add, Value.String(lhs), Value.String(rhs)) => Value.String(lhs + rhs),

        (BinaryOp.Eq, lhs, rhs) => Value.Bool(lhs == rhs),
        (BinaryOp.NotEq, lhs, rhs) => Value.Bool(lhs != rhs),

        (op, lhs, rhs) => todo(),
    },
    Expr.Call(lhs, args) => match todo(lhs.eval(env)) {
        Value.Function(inner) => {
            let mut arg_values = Vec.new();
            for expr in args {
                arg_values.push(todo(expr.eval(env)));
            }
            inner.call(env, arg_values)
        }
        Value.NativeFunction(inner) => {
            let mut arg_values = Vec.new();
            for expr in args {
                arg_values.push(todo(expr.eval(env)));
            }
            inner.call(env, arg_values)
        }
        _ => todo("Add error, value is not a function."),
    },

    Expr.IfElse(cond_expr, then_expr, else_expr) => {
        let Value.Bool(condition) = todo(cond_expr.eval(env)) else {
            todo();
        };
        if condition then
            todo(then_expr.eval(env))
        else
            todo(else_expr.eval(env))
    }
    Expr.Block(exprs) => {
        env.push_scope(ScopeKind.Expr);
        let mut result = Value.Unit;
        #exprs_loop: for expr in exprs {
            match expr.eval(env) {
                ControlFlow.None(_) => (),
                ControlFlow.Break(inner) => {
                    result = inner;
                    break #exprs_loop;
                }
                control_flow => return control_flow,
            }
        }
        env.pop_scope();
        result
    }
    Expr.Break(expr) => return ControlFlow.Break(todo(expr.eval(env))),
    Expr.Return(expr) => return ControlFlow.Return(todo(expr.eval(env))),

    Expr.Var(kind, name, expr) => {
        let val = todo(expr.eval(env));
        env.create(name, kind, val);
        Value.Unit
    }
}
```
