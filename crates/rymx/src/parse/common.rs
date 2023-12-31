use super::{error::ParseError, type_parser};
use crate::{ast::*, tokenize::Token, Span};
use chumsky::{
    input::{MapExtra, SpannedInput},
    prelude::*,
};

pub(super) type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
pub(super) type Extra<'src> = extra::Full<ParseError, (), &'src str>;
// pub(super) type MyParser<'src, 'token, O> = Parser<'src, TokenStream<'token>, O, Extra<'src>>;

pub(super) fn parameters_parser<'src>(
) -> impl Parser<'src, TokenStream<'src>, Vec<(String, Type)>, Extra<'src>> + Clone {
    // parameter ::= ident (":" type)?
    let parameter = ident_parser()
        .then(just(Token::Colon).ignore_then(type_parser()).or_not())
        .map(|(name, maybe_typ)| (name.to_string(), maybe_typ.unwrap_or(Type::Unkown)))
        .labelled("parameter");

    // parameters ::= (parameter ("," parameter)*)?
    parameter
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
}

pub fn path_parser<'src>() -> impl Parser<'src, TokenStream<'src>, Path, Extra<'src>> + Clone {
    // path ::= ident ("." ident)*
    ident_parser()
        .map(String::from)
        .separated_by(just(Token::Dot))
        .at_least(1)
        .collect::<Vec<String>>()
        .map(Path::new)
}

pub fn literal_parser<'src>() -> impl Parser<'src, TokenStream<'src>, Literal, Extra<'src>> + Clone
{
    let integer = just(Token::Int)
        .map_with(|_, extra| {
            Literal::Int(
                source(current_span(extra), extra)
                    .parse()
                    .expect("Internal Error: Failed to parse i64"),
            )
        })
        .labelled("integer");

    let float = just(Token::Float)
        .map_with(|_, extra| {
            Literal::Float(
                source(current_span(extra), extra)
                    .parse()
                    .expect("Internal Error: Failed to parse f64"),
            )
        })
        .labelled("float");

    let string = just(Token::String)
        .map_with(|_, extra| {
            Literal::String({
                let mut span: Span = current_span(extra);
                span.start += 1;
                span.end -= 1;
                source(span, extra).into()
            })
        })
        .labelled("string");

    choice((integer, float, string)).labelled("literal").boxed()
}

pub(super) fn ident_parser<'src>(
) -> impl Parser<'src, TokenStream<'src>, &'src str, Extra<'src>> + Clone {
    just(Token::Ident)
        .map_with(|_, extra| source(current_span(extra), extra))
        .labelled("identifier")
}

pub(super) fn current_span<'a>(extra: &mut MapExtra<'a, '_, TokenStream<'a>, Extra<'a>>) -> Span {
    extra.span()
}

pub(super) fn source<'a>(
    span: Span,
    extra: &mut MapExtra<'a, '_, TokenStream<'a>, Extra<'a>>,
) -> &'a str {
    span.src(extra.ctx())
}
