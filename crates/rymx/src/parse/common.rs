use crate::{ast::Expr, tokenize::Token, Span};
use chumsky::{
	input::{MapExtra, SpannedInput},
	prelude::*,
};

pub(super) type TokenStream<'tokens> = SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
pub(super) type Extra<'src> = extra::Full<Rich<'src, Token, Span>, (), &'src str>;

pub(super) enum FnKind {
	// All types except the return type must be written down
	// default return type is unit
	Stmt,
	// Types will be inferred
	// default return type is inferred/unkown
	Expr,
}
pub(super) fn make_function(
	fn_kind: FnKind,
	name: Option<&str>,
	params: Vec<&str>,
	return_type: Option<&str>,
	body: Expr,
) -> crate::interpret::Function {
	crate::interpret::Function {
		name: name.map(String::from),
		params: params.into_iter().map(|name| name.to_string()).collect(),
		return_type: match fn_kind {
			FnKind::Stmt => return_type.unwrap_or("()").into(),
			FnKind::Expr => return_type.unwrap_or("Unknown").into(),
		},
		body: Box::new(body),
	}
}

pub(super) fn parameters_parser<'src>(
	src: &'src str,
) -> impl Parser<TokenStream, Vec<&str>, Extra> + Clone {
	// parameter ::= ident (":" __TODO__)?
	let parameter = ident_parser(src)
		.then(just(Token::Colon).ignore_then(ident_parser(src)).or_not())
		.map(|(name, _typ)| name)
		.labelled("parameter");

	// parameters ::= (parameter ("," parameter)*)?
	let parameters = parameter
		.separated_by(just(Token::Comma))
		.allow_trailing()
		.collect::<Vec<&str>>();

	parameters
}

pub(super) fn block_parser<'src>(
	expr: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone + 'src,
	function_stmt: impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone + 'src,
) -> impl Parser<'src, TokenStream<'src>, Expr, Extra<'src>> + Clone {
	// statement ::= expr ";" | function
	let statement = choice((expr.clone().then_ignore(just(Token::Semi)), function_stmt));

	// block ::= "{" statement* expr? "}"
	statement
		.repeated()
		.collect::<Vec<Expr>>()
		.then(expr.clone().or_not())
		.delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
		.map(|(mut exprs, last)| {
			if let Some(last) = last {
				if !matches!(last, Expr::Return(..) | Expr::Break(..)) {
					exprs.push(Expr::Break(Box::new(last)));
				}
			}
			Expr::Block(exprs)
		})
		.boxed()
}

pub(super) fn ident_parser(src: &str) -> impl Parser<TokenStream, &str, Extra> + Clone {
	just(Token::Ident)
		.map_with(|_, extra| current_src(extra, src))
		.labelled("indentifier")
}

/// Retrieve the substring of source code at the current span.
pub(super) fn current_src<'src>(
	extra: &mut MapExtra<'src, '_, TokenStream<'src>, Extra<'src>>,
	src: &'src str,
) -> &'src str {
	extra.span().src(src)
}
