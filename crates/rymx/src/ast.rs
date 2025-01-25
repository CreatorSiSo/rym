use itertools::Itertools;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub name: String,
    pub constants: &'a [(String, Type, Expr<'a>)],
    pub types: Vec<(String, Type)>,
    pub sub_modules: Vec<Module<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Function(Function<'a>),
    Type(String, Type),
    Variable(VariableKind, String, Type, Expr<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableKind {
    Const,
    Let,
    LetMut,
}

impl Display for VariableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            VariableKind::Const => "const",
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
    pub params: &'a [(String, Type)],
    pub named_params: &'a [(String, Type, Expr<'a>)],
    pub return_type: Type,
    pub body: &'a Expr<'a>,
}

impl PartialEq for Function<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.params
            .iter()
            .zip(other.params.iter())
            .all(|((_, typ0), (_, typ1))| typ0 == typ1)
            && self.named_params == other.named_params
            && self.return_type == other.return_type
            && self.body == other.body
    }
}

impl Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "fn({}) {}",
            self.params
                .iter()
                .map(|(_, typ)| format!("{typ}"))
                .chain(
                    self.named_params
                        .iter()
                        .map(|(name, typ, expr)| format!("{name}: {typ} = {expr:?}"))
                )
                .join(", "),
            self.return_type
        ))
    }
}

// TODO comments
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Unkown,
    Never,
    Literal(Literal),
    Path(Path),
    Generic(Box<Type>, Vec<Type>),
    Function {
        args: Vec<Type>,
        named_args: Vec<(String, Type, Literal)>,
        return_type: Box<Type>,
    },
    Array(ArraySize, Box<Type>),
    Struct(Vec<(String, Type, Option<Literal>)>),
    Enum(Vec<(String, Option<Type>)>),
    Union(Vec<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "()"),
            Type::Unkown => write!(f, "<unknown>"),
            Type::Never => write!(f, "<never>"),
            Type::Literal(lit) => write!(f, "{lit}"),
            Type::Path(path) => write!(f, "{path}"),
            Type::Generic(typ, args) => write!(f, "{typ}[{}]", args.iter().join(", ")),
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
                            .map(|(name, typ, val)| format!("{name}: {typ} = {val}"))
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
                    .map(|(name, typ, maybe_val)| format!(
                        "{name}: {typ}{}",
                        maybe_val
                            .as_ref()
                            .map(|val| " = ".to_string() + &val.to_string())
                            .unwrap_or("".into())
                    ))
                    .join(", ")
            ),
            Type::Enum(variants) => write!(
                f,
                "enum {}",
                variants
                    .iter()
                    .map(|(name, maybe_typ)| format!(
                        "{name}{}",
                        maybe_typ
                            .as_ref()
                            .map(|typ| " ".to_string() + &typ.to_string())
                            .unwrap_or("".to_string())
                    ))
                    .join(" | ")
            ),
            Type::Union(types) => write!(f, "union {}", types.iter().join(" | ")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArraySize {
    Unknown,
    Path(Path),
    Int(u64),
}

impl Display for ArraySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArraySize::Unknown => write!(f, ""),
            ArraySize::Path(path) => write!(f, "{path}"),
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
