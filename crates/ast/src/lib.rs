use itertools::Itertools;
use span::Span;
use std::fmt::{Debug, Display};

pub type SpannedExpr = Expr<Span>;
pub type SpannedStmt = Stmt<Span>;
pub type SpannedFunction = Function<Span>;

#[derive(Debug, Clone, PartialEq)]
pub struct Typed {
    pub span: Span,
    pub typ: Type,
}

impl Typed {
    pub fn new(span: Span, typ: Type) -> Self {
        Self { span, typ }
    }
}

impl Display for Typed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.typ)
    }
}

pub type TypedExpr = Expr<Typed>;
pub type TypedStmt = Stmt<Typed>;
pub type TypedFunction = Function<Typed>;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub sub_modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<Extra: Display + Clone> {
    Expr(Expr<Extra>),
    Function(Function<Extra>),
    Variable(VariableKind, String, Type, Expr<Extra>),
}

impl<Extra: Display + Clone> Display for Stmt<Extra> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{expr}"),
            Stmt::Function(function) => write!(f, "{function}"),
            Stmt::Variable(variable_kind, name, typ, expr) => {
                write!(f, "{variable_kind} {name}: {typ} = {expr}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableKind {
    Let,
    LetMut,
}

impl Display for VariableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            VariableKind::Let => "let",
            VariableKind::LetMut => "let mut",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<Extra: Display + Clone> {
    pub kind: ExprKind<Extra>,
    pub extra: Extra,
}

impl<Extra: Display + Clone> Display for Expr<Extra> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.kind, self.extra)
    }
}

pub type ExprRef<Extra> = Box<Expr<Extra>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<Extra: Display + Clone> {
    Error,
    Unit,

    // Value creation
    Literal(Literal),
    Array(Vec<Expr<Extra>>),
    ArrayWithRepeat(ExprRef<Extra>, ExprRef<Extra>),
    Tuple(Vec<ExprRef<Extra>>),
    Struct(Path, Vec<(String, ExprRef<Extra>)>),
    Function(Function<Extra>),

    // Value modification
    Unary(UnaryOp, ExprRef<Extra>),
    Binary(BinaryOp, ExprRef<Extra>, ExprRef<Extra>),
    Call(ExprRef<Extra>, Vec<Expr<Extra>>),

    // Value access
    Ident(String),
    Subscript(ExprRef<Extra>, ExprRef<Extra>),
    FieldAccess(ExprRef<Extra>, String),

    // Control flow
    IfElse(
        /// Condition
        ExprRef<Extra>,
        /// Then branch
        ExprRef<Extra>,
        /// Else branch
        ExprRef<Extra>,
    ),
    Block(Vec<Stmt<Extra>>),
    Break(ExprRef<Extra>),
    Return(ExprRef<Extra>),
}

impl<E: Display + Clone> Display for ExprKind<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "<error>"),
            Self::Unit => write!(f, "Unit"),
            Self::Literal(arg0) => write!(f, "{arg0}"),
            Self::Array(exprs) => {
                write!(f, "[{}]", exprs.iter().map(ToString::to_string).join(", "))
            }
            Self::ArrayWithRepeat(arg0, arg1) => write!(f, "[{arg0}; {arg1}]"),
            Self::Tuple(arg0) => write!(f, "({})", arg0.iter().map(ToString::to_string).join(", ")),
            Self::Struct(path, fields) => write!(
                f,
                "{path:?} {{\n{}}}",
                fields
                    .iter()
                    .map(|(name, expr)| format!("{name} = {expr},\n"))
                    .collect::<String>()
            ),
            Self::Function(func) => write!(f, "{func}"),

            Self::Unary(op, expr) => write!(f, "{op}({expr})"),
            Self::Binary(op, a, b) => write!(f, "({a}) {op} ({b})"),
            Self::Call(func, args) => write!(
                f,
                "({func})({})",
                args.iter().map(ToString::to_string).join(", ")
            ),

            Self::Subscript(arg0, arg1) => write!(f, "{arg0}[{arg1}]"),
            Self::FieldAccess(arg0, arg1) => write!(f, "({arg0}).{arg1}"),
            Self::Ident(ident) => write!(f, "{ident}"),

            Self::IfElse(cond, then, r#else) => {
                write!(f, "if {cond} {{ {then} }} else {{ {} }}", r#else)
            }
            Self::Block(stmts) => {
                writeln!(f, "{{")?;
                for stmt in stmts {
                    writeln!(f, "  {stmt}")?;
                }
                write!(f, "}}")
            }
            Self::Break(arg0) => write!(f, "break {arg0}"),
            Self::Return(arg0) => write!(f, "return {arg0}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function<Extra: Display + Clone> {
    pub name: Option<String>,
    pub params: Vec<FunctionParam<Extra>>,
    pub return_type: Type,
    pub body: ExprRef<Extra>,
}

impl<Extra: PartialEq + Display + Clone> PartialEq for Function<Extra> {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}

impl<Extra: Display + Clone> Display for Function<Extra> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "fn {name}")?;
        } else {
            write!(f, "fn")?;
        }
        write!(
            f,
            "({}) -> {} {}",
            self.params.iter().map(FunctionParam::to_string).join(", "),
            self.return_type,
            self.body
        )
    }
}

#[derive(Debug, Clone)]
pub struct FunctionParam<Extra: Display + Clone> {
    pub name: String,
    pub typ: Type,
    pub default_value: Option<Expr<Extra>>,
}

impl<E: PartialEq + Display + Clone> PartialEq for FunctionParam<E> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.typ == other.typ
    }
}

impl<E: Display + Clone> Display for FunctionParam<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(param) = &self.default_value {
            write!(f, "{}: {} = {param}", self.name, self.typ)
        } else {
            write!(f, "{}: {}", self.name, self.typ)
        }
    }
}

// TODO comments
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Unkown,
    Never,
    Bool,
    Int,
    Float,
    String,
    Function {
        params: Vec<(String, Type)>,
        result: Box<Type>,
    },
    Array {
        len: Option<u64>,
        element: Box<Type>,
    },
    Struct(Vec<(String, Type)>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::Unkown => write!(f, "<unknown>"),
            Type::Never => write!(f, "!"),
            Type::Bool => write!(f, "Bool"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Function { params, result } => write!(
                f,
                "fn({}) -> {result}",
                params
                    .iter()
                    .map(|(name, typ)| format!("{name}: {typ}"))
                    .join(", "),
            ),
            Type::Array {
                len: Some(len),
                element,
            } => write!(f, "[{len}]{element}",),
            Type::Array { len: None, element } => write!(f, "[]{element}",),
            Type::Struct(fields) => write!(
                f,
                "struct {{{0}{1}{0}}}",
                if fields.is_empty() { "" } else { " " },
                fields
                    .iter()
                    .map(|(name, typ)| format!("{name}: {typ}",))
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArraySize {
    Unknown,
    Int(u64),
}

impl Display for ArraySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArraySize::Unknown => write!(f, ""),
            ArraySize::Int(int) => write!(f, "{int}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Negation `-1`
    Neg,
    /// Not `not true`
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition `1 + 2`
    Add,
    /// Subtraction `1 - 2`
    Sub,
    /// Multiplication `1 * 2`
    Mul,
    /// Division `1 / 2`
    Div,

    /// Equality `1 == 2`
    Eq,
    /// Inequality `1 != 2`
    NotEq,
    /// Less than `1 < 2`
    LessThan,
    /// Less than or equal `1 <= 2`
    LessThanEq,
    /// Greater than `1 > 2`
    GreaterThan,
    /// Less than or equal `1 >= 2`
    GreaterThanEq,

    // Assignment `tmp = 0`
    Assign,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Path {
    pub parts: Vec<String>,
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path({:?})", self.parts)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.parts.join("."))
    }
}

impl Path {
    pub fn new(parts: Vec<String>) -> Self {
        Self { parts }
    }
}

#[derive(Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0) => f.write_fmt(format_args!("Int: {arg0}")),
            Self::Float(arg0) => f.write_fmt(format_args!("Float: {arg0}")),
            Self::String(arg0) => f.write_fmt(format_args!("String: {arg0:?}")),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Literal::Int(inner) => inner.to_string(),
            Literal::Float(inner) => inner.to_string(),
            Literal::String(inner) => inner.to_string(),
        })
    }
}
