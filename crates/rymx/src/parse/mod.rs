// use self::error::{ParseError, Pattern};
use crate::{error::SourceId, span::Span, tokenize::Token};
use chumsky::{
    extra::Full,
    input::{Input, MapExtra, ValueInput},
    prelude::*,
};

type Error<'src> = Rich<'src, Token, Span>;
type Extra<'src> = Full<Error<'src>, (), &'src str>;

pub fn parse_file<'src>(
    tokens: &[(Token, Span)],
    src: &'src str,
    src_id: SourceId,
) -> Vec<ast::Function> {
    with_src(module(), src)
        .parse(tokens.map(
            Span {
                start: src.len(),
                end: src.len(),
                id: src_id,
            },
            |(token, span)| (token, span),
        ))
        .unwrap()
}

fn with_src<'src, I>(
    parser: impl Parser<'src, I, Vec<ast::Function>, Extra<'src>>,
    src: &'src str,
) -> impl Parser<'src, I, Vec<ast::Function>, Extra<'src>>
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    parser.with_ctx(src)
}

fn module<'src, I>() -> impl Parser<'src, I, Vec<ast::Function>, Extra<'src>>
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    function().repeated().collect()
}

fn function<'src, I>() -> impl Parser<'src, I, ast::Function, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::Fn)
        .ignore_then(ident().or_not())
        .then(
            parameter()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<ast::FunctionParam>>()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
        )
        .then(just(Token::ThinArrow).ignore_then(typ()).or_not())
        .then(block().or_not())
        .map(|(((name, params), return_type), body)| ast::Function {
            name: name.map(String::from),
            params,
            return_type: return_type.unwrap_or(ast::Type::Unit),
            body: Box::new(body.map(ast::Expr::Block).unwrap_or(ast::Expr::Error)),
        })
}

fn block<'src, I>() -> impl Parser<'src, I, Vec<ast::Stmt>, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    statement()
        .repeated()
        .collect()
        .delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
}

fn statement<'src, I>() -> impl Parser<'src, I, ast::Stmt, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    choice((
        expression().map(ast::Stmt::Expr),
        just(Token::Let)
            .ignore_then(just(Token::Mut).or_not())
            .then(ident())
            .then(just(Token::Colon).ignore_then(typ()).or_not())
            .then_ignore(just(Token::Assign))
            .then(expression())
            .map(|(((kind, ident), typ), expr)| {
                let kind = match kind {
                    Some(_) => ast::VariableKind::LetMut,
                    None => ast::VariableKind::Let,
                };
                ast::Stmt::Variable(
                    kind,
                    ident.to_owned(),
                    typ.unwrap_or(ast::Type::Unkown),
                    expr,
                )
            }),
    ))
}

fn expression<'src, I>() -> impl Parser<'src, I, ast::Expr, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    recursive(|expr| {
        let literal = literal().map(ast::Expr::Literal);
        let ident = ident().map(|ident| ast::Expr::Ident(ident.to_owned()));

        let atom = choice((
            literal,
            ident,
            expr.clone()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
        ));

        let arguments = expr
            .clone()
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<ast::Expr>>();

        let call = atom.foldl(
            arguments
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
                .repeated(),
            |func, args| ast::Expr::Call(Box::new(func), args),
        );

        let if_else = just(Token::If)
            .ignore_then(expr.clone())
            .then(expr.clone())
            .then(just(Token::Else).ignore_then(expr.clone()).or_not())
            .map(|((cond, a), b)| {
                ast::Expr::IfElse(
                    Box::new(cond),
                    Box::new(a),
                    Box::new(b.unwrap_or(ast::Expr::Block(Vec::new()))),
                )
            });

        choice((call, if_else))
    })
}

fn parameter<'src, I>() -> impl Parser<'src, I, ast::FunctionParam, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    ident()
        .then_ignore(just(Token::Colon))
        .then(typ())
        .map(|(name, typ)| ast::FunctionParam {
            name: name.to_owned(),
            typ,
            default_value: None,
        })
}

fn typ<'src, I>() -> impl Parser<'src, I, ast::Type, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    recursive(|typ| {
        choice((
            ident().map(|ident| match ident {
                "Unit" => ast::Type::Unit,
                "Int" => ast::Type::Int,
                "Float" => ast::Type::Float,
                "String" => ast::Type::String,
                _ => ast::Type::Unkown,
            }),
            just(Token::Exclamation).map(|_| ast::Type::Never),
            just(Token::BracketOpen)
                .ignore_then(int().or_not())
                .then_ignore(just(Token::BracketClose))
                .then(typ)
                .map(|(len, element)| ast::Type::Array {
                    len,
                    element: Box::new(element),
                }),
        ))
    })
}

fn literal<'src, I>() -> impl Parser<'src, I, ast::Literal, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    choice((
        int().map(|uint| ast::Literal::Int(uint as i64)),
        float().map(ast::Literal::Float),
        string().map(ast::Literal::String),
    ))
}

fn int<'src, I>() -> impl Parser<'src, I, u64, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::Int).map_with(|_, extra| src_at(extra).parse::<u64>().unwrap())
}

fn float<'src, I>() -> impl Parser<'src, I, f64, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::Float).map_with(|_, extra| src_at(extra).parse::<f64>().unwrap())
}

fn string<'src, I>() -> impl Parser<'src, I, String, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::String).map_with(|_, extra| src_at(extra).to_owned())
}

fn ident<'src, I>() -> impl Parser<'src, I, &'src str, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::Ident).map_with(|_, extra| src_at(extra))
}

fn src_at<'src, I>(extra: &mut MapExtra<'src, '_, I, Extra<'src>>) -> &'src str
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    Span::src(extra.span(), *extra.ctx())
}
