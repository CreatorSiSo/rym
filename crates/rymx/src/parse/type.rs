use super::common::*;
use crate::{ast::*, tokenize::Token};
use chumsky::prelude::*;

pub fn typedef_parser(src: &str) -> impl Parser<TokenStream, (&str, Type), Extra> + Clone {
	// typedef ::=  "type" ident "=" type ";")
	let typedef = just(Token::Type)
		.ignore_then(ident_parser(src))
		.then_ignore(just(Token::Assign))
		.then(type_parser(src))
		.then_ignore(just(Token::Semi))
		.labelled("type definition");

	typedef
}

pub fn type_parser(src: &str) -> impl Parser<TokenStream, Type, Extra> + Clone {
	recursive(|type_| {
		// literal ::= int | float | string
		let literal = literal_parser(src).map(Type::Literal);

		let ident = ident_parser(src).map(String::from);

		// path ::= ident ("." ident)*
		let path = ident
			.clone()
			.separated_by(just(Token::Dot))
			.at_least(1)
			.collect::<Vec<String>>()
			.map(Path::new);

		// atom ::= "(" ")" | literal | path | "(" type ")"
		let atom = choice((
			just(Token::ParenOpen)
				.then(just(Token::ParenClose))
				.to(Type::Unit),
			literal,
			path.clone().map(Type::Path),
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
			.ignore_then(struct_fields.delimited_by(just(Token::BraceOpen), just(Token::BraceClose)))
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

		// size ::= (path | int)
		let size = path
			.map(ArraySize::Path)
			.or(integer_parser(src).map(|int| ArraySize::Int(int as u64)));
		// array ::= "[" size? "]" type
		let array = (size.or_not())
			.delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
			.then(type_)
			.map(|(size, element)| Type::Array(size, Box::new(element)));

		choice((struct_, enum_, array, generic))
	})
	.labelled("type")
}
