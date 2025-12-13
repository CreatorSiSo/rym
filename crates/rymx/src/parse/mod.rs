mod error;

use crate::{error::Diagnostic, parse::error::map_parse_result, tokenize::Token};
use ast::{Expr, ExprKind, SpannedExpr, SpannedFunction, SpannedStmt};
use chumsky::{
    extra::Full,
    input::{Input, MapExtra, ValueInput},
    prelude::*,
};
use span::{SourceId, Span};

type Error<'src> = Rich<'src, Token, Span>;
type Extra<'src> = Full<Error<'src>, (), &'src str>;

pub fn parse_file<'src>(
    tokens: &[(Token, Span)],
    src: &'src str,
    src_id: SourceId,
) -> Result<Vec<SpannedFunction>, Vec<Diagnostic>> {
    let result = with_src(module(), src).parse(tokens.map(
        Span {
            start: src.len(),
            end: src.len(),
            id: src_id,
        },
        |(token, span)| (token, span),
    ));

    map_parse_result(result)
}

fn with_src<'src, I>(
    parser: impl Parser<'src, I, Vec<SpannedFunction>, Extra<'src>>,
    src: &'src str,
) -> impl Parser<'src, I, Vec<SpannedFunction>, Extra<'src>>
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    parser.with_ctx(src)
}

fn module<'src, I>() -> impl Parser<'src, I, Vec<SpannedFunction>, Extra<'src>>
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    function().repeated().collect()
}

fn function<'src, I>() -> impl Parser<'src, I, SpannedFunction, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    just(Token::Fn)
        .ignore_then(ident().or_not())
        .then(
            parameter()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<ast::FunctionParam<Span>>>()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
        )
        .then(just(Token::ThinArrow).ignore_then(typ()).or_not())
        .then(
            block()
                .or_not()
                .map_with(|body, extra| (body, extra.span())),
        )
        .map(
            |(((name, params), return_type), (body, body_span))| SpannedFunction {
                name: name.map(String::from),
                params,
                return_type: return_type.unwrap_or(ast::Type::Unit),
                body: Box::new(Expr {
                    kind: body
                        .map(ast::ExprKind::Block)
                        .unwrap_or(ast::ExprKind::Error),
                    extra: body_span,
                }),
            },
        )
}

fn block<'src, I>() -> impl Parser<'src, I, Vec<SpannedStmt>, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    statement()
        .repeated()
        .collect()
        .delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
}

fn statement<'src, I>() -> impl Parser<'src, I, SpannedStmt, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    choice((
        expression().map(SpannedStmt::Expr),
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

fn expression<'src, I>() -> impl Parser<'src, I, SpannedExpr, Extra<'src>> + Clone
where
    I: ValueInput<'src, Span = Span, Token = Token>,
{
    recursive(|expr| {
        let literal = literal().map_with(|lit, extra| Expr {
            kind: ExprKind::Literal(lit),
            extra: extra.span(),
        });
        let ident = ident().map_with(|ident, extra| Expr {
            kind: ExprKind::Ident(ident.to_owned()),
            extra: extra.span(),
        });

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
            .collect::<Vec<SpannedExpr>>();

        let call = atom.foldl_with(
            arguments
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose))
                .repeated(),
            |func, args, extra| Expr {
                kind: ExprKind::Call(Box::new(func), args),
                extra: extra.span(),
            },
        );

        let if_else = just(Token::If)
            .ignore_then(expr.clone())
            .then(expr.clone())
            .then(just(Token::Else).ignore_then(expr.clone()).or_not())
            .map_with(|((cond, a), b), extra| {
                let span_a = a.extra;
                Expr {
                    kind: ExprKind::IfElse(
                        Box::new(cond),
                        Box::new(a),
                        Box::new(b.unwrap_or(Expr {
                            kind: ExprKind::Block(Vec::new()),
                            extra: Span::new(span_a.end, span_a.end).with_id(span_a.id),
                        })),
                    ),
                    extra: extra.span(),
                }
            });

        choice((call, if_else))
    })
}

fn parameter<'src, I>() -> impl Parser<'src, I, ast::FunctionParam<Span>, Extra<'src>> + Clone
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
