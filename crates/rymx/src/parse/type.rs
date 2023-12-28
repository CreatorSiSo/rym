use super::common::*;
use crate::{ast::*, tokenize::Token};
use chumsky::prelude::*;

pub fn type_parser(src: &str) -> impl Parser<TokenStream, Type, Extra> + Clone {
    recursive(|type_| {
        // literal ::= int | float | string
        let literal = literal_parser(src).map(Type::Literal);

        let ident = ident_parser(src).map(String::from);

        // atom ::= "(" ")" | literal | path | "(" type ")"
        let atom = choice((
            just(Token::ParenOpen)
                .then(just(Token::ParenClose))
                .to(Type::Unit),
            literal,
            path_parser(src).map(Type::Path),
            type_
                .clone()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
        ))
        .labelled("atom");

        // generic ::= atom ("[" type ("," type)* ","? "]")?
        let generic = atom
            .then(
                type_
                    .clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .collect::<Vec<Type>>()
                    .delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
                    .or_not(),
            )
            .map(|(typ, args)| match args {
                Some(args) => Type::Generic(Box::new(typ), args),
                None => typ,
            });

        // struct_field ::= ident ":" type ("=" literal)?
        let struct_field = ident
            .clone()
            .then_ignore(just(Token::Colon))
            .then(type_.clone())
            .then(
                just(Token::Assign)
                    .ignore_then(literal_parser(src))
                    .or_not(),
            )
            .map(|((name, typ), default)| (name, typ, default));
        // struct_fields ::= (struct_field ",")* struct_field?
        let struct_fields = struct_field
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<(String, Type, Option<Literal>)>>();
        // struct ::= struct "{" struct_fields "}"
        let struct_ = just(Token::Struct)
            .ignore_then(
                struct_fields.delimited_by(just(Token::BraceOpen), just(Token::BraceClose)),
            )
            .map(Type::Struct)
            .labelled("struct");

        // enum_variant ::= ident type?
        let enum_variant = ident.then(type_.clone().or_not());
        // enum_variants ::= "|"? enum_variant ("|" enum_variant)*
        let enum_variants = enum_variant
            .separated_by(just(Token::Pipe))
            .allow_leading()
            .collect::<Vec<(String, Option<Type>)>>();
        // enum ::= "enum" enum_variants
        let enum_ = just(Token::Enum)
            .ignore_then(enum_variants)
            .map(Type::Enum)
            .labelled("enum");

        // union_variants ::= "|"? type ("|" type)*
        let union_variants = type_
            .clone()
            .separated_by(just(Token::Pipe))
            .allow_leading()
            .collect::<Vec<Type>>();
        // union ::= "union" union_variants
        let union = just(Token::Union)
            .ignore_then(union_variants)
            .map(Type::Union)
            .labelled("union");

        // size ::= (path | int)
        let size = path_parser(src)
            .map(ArraySize::Path)
            .or(integer_parser(src).map(|int| ArraySize::Int(int as u64)));
        // array ::= "[" size? "]" type
        let array = (size.or_not())
            .delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
            .then(type_)
            .map(|(size, element)| {
                Type::Array(size.unwrap_or(ArraySize::Unknown), Box::new(element))
            });

        choice((struct_, enum_, union, array, generic))
    })
    .labelled("type")
}