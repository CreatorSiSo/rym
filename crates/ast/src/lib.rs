use itertools::Itertools;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub sub_modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Function(Function<'a>),
    Variable(VariableKind, String, Type, Expr<'a>),
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

#[derive(Clone, PartialEq)]
pub enum Expr<'a> {
    // Value creation
    Literal(Literal),
    Array(&'a [Expr<'a>]),
    ArrayWithRepeat(&'a Expr<'a>, &'a Expr<'a>),
    Tuple(&'a [Expr<'a>]),
    Struct(Path, &'a [(String, Expr<'a>)]),
    Function(Function<'a>),

    // Value modification
    Unary(UnaryOp, &'a Expr<'a>),
    Binary(BinaryOp, &'a Expr<'a>, &'a Expr<'a>),
    Call(&'a Expr<'a>, &'a [Expr<'a>]),

    // Value access
    Ident(String),
    Subscript(&'a Expr<'a>, &'a Expr<'a>),
    FieldAccess(&'a Expr<'a>, String),

    // Control flow
    IfElse(
        /// Condition
        &'a Expr<'a>,
        /// Then branch
        &'a Expr<'a>,
        /// Else branch
        &'a Expr<'a>,
    ),
    Block(Vec<Stmt<'a>>),
    Break(&'a Expr<'a>),
    Return(&'a Expr<'a>),
}

impl std::fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => f.write_fmt(format_args!("Literal({arg0:?})")),
            Self::Array(arg0) => f.write_fmt(format_args!("Array({arg0:?})")),
            Self::ArrayWithRepeat(arg0, arg1) => f
                .debug_tuple("ArrayWithRepeat")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Struct(arg0, arg1) => f.debug_tuple("Struct").field(arg0).field(arg1).finish(),
            Self::Function(arg0) => f.write_fmt(format_args!("{arg0:#?}")),

            Self::Unary(arg0, arg1) => f.debug_tuple(&arg0.to_string()).field(arg1).finish(),
            Self::Binary(arg0, arg1, arg2) => f
                .debug_tuple(&arg0.to_string())
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Call(arg0, arg1) => f.debug_tuple("Call").field(arg0).field(arg1).finish(),

            Self::Subscript(arg0, arg1) => {
                f.debug_tuple("Subscript").field(arg0).field(arg1).finish()
            }
            Self::FieldAccess(arg0, arg1) => f
                .debug_tuple("FieldAccess")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Ident(arg0) => f.write_fmt(format_args!("Ident({arg0:?})")),

            Self::IfElse(arg0, arg1, arg2) => f
                .debug_tuple("IfElse")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Break(arg0) => f.debug_tuple("Break").field(arg0).finish(),
            Self::Return(arg0) => f.debug_tuple("Return").field(arg0).finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub params: &'a [FunctionParam<'a>],
    pub return_type: Type,
    pub body: &'a Expr<'a>,
}

impl PartialEq for Function<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.return_type == other.return_type
    }
}

impl Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}) -> {}",
            self.params.iter().map(FunctionParam::to_string).join(", "),
            self.return_type
        ))
    }
}

#[derive(Debug, Clone)]
pub struct FunctionParam<'a> {
    name: String,
    typ: Type,
    default_value: Option<Expr<'a>>,
}

impl PartialEq for FunctionParam<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.typ == other.typ
    }
}

impl Display for FunctionParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(param) = &self.default_value {
            write!(f, "{}: {} = {param:?}", self.name, self.typ)
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
    Literal(Literal),
    Function {
        args: Vec<Type>,
        named_args: Vec<(String, Type)>,
        return_type: Box<Type>,
    },
    Array(ArraySize, Box<Type>),
    Struct(Vec<(String, Type)>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "()"),
            Type::Unkown => write!(f, "<unknown>"),
            Type::Never => write!(f, "<never>"),
            Type::Literal(lit) => write!(f, "{lit}"),
            Type::Function {
                args,
                named_args,
                return_type,
            } => write!(
                f,
                "fn({}) {return_type}",
                args.iter()
                    .map(Type::to_string)
                    .chain(
                        named_args
                            .iter()
                            .map(|(name, typ)| format!("{name}: {typ}"))
                    )
                    .join(", "),
            ),
            Type::Array(size, typ) => write!(f, "[{size}]{typ}",),
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
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(arg0) => f.write_fmt(format_args!("Bool: {arg0}")),
            Self::Int(arg0) => f.write_fmt(format_args!("Int: {arg0}")),
            Self::Float(arg0) => f.write_fmt(format_args!("Float: {arg0}")),
            Self::String(arg0) => f.write_fmt(format_args!("String: {arg0:?}")),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Literal::Bool(inner) => inner.to_string(),
            Literal::Int(inner) => inner.to_string(),
            Literal::Float(inner) => inner.to_string(),
            Literal::String(inner) => inner.to_string(),
        })
    }
}
