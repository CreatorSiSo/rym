extern crate proc_macro;

use std::fmt::Display;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

#[proc_macro]
pub fn make_ast(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut input = TokenStream::from(input).into_iter();
	let mut output = TokenStream::new();

	// let ___v: Vec<Stmt>;
	output.append_all(quote!(let ___v: Vec<Stmt>;));

	// __v = vec![ /* generated ast */ ];
	output.append(Ident::new("___v", Span::call_site()));
	output.append(Punct::new('=', Spacing::Alone));
	output.append(Ident::new("vec", Span::call_site()));
	output.append(Punct::new('!', Spacing::Joint));
	output.append(Group::new(Delimiter::Bracket, stmts(&mut input)));
	output.append(Punct::new(';', Spacing::Joint));

	// __v
	output.append(Ident::new("___v", Span::call_site()));

	// { .. }
	Group::new(Delimiter::Brace, output)
		.into_token_stream()
		.into()
}

fn stmts<T: Iterator<Item = TokenTree>>(input: &mut T) -> TokenStream {
	let mut stmts: Vec<TokenStream> = Vec::new();
	let mut push = |stream: TokenStream| stmts.push(stream);

	while let Some(token) = input.next() {
		match token.to_string().as_str() {
			"Empty" => push(quote!(Stmt::Empty)),
			"Expr" => push(expr(&token, input)), // TODO Make the Expr statement work properly
			"Decl" => push(decl(&token, input)),
			_ => push(make_error("Expected `Empty | Expr | Decl`", Show(&token))),
		};

		// TODO Make commas work
		// comma(&mut input);
	}

	let mut output = TokenStream::new();
	output.append_separated(stmts.into_iter(), Punct::new(',', Spacing::Alone));
	output
}

/*
fn comma<T: Iterator<Item = TokenTree>>(tokens: &mut T) -> TokenStream {
	match tokens.next() {
		Some(token) => match token {
			TokenTree::Punct(punct) => {
				if punct.as_char() != ',' {
					return make_error(&punct, &format!("Expected `,` got `{punct}`"));
				}
			}
			tt => return make_error(&tt, &format!("Expected `,` got `{tt}`")),
		},
		None => {
			if let Some(tt) = tokens.next() {
				return make_error(&tt, &format!("Expected `,` got `{tt}`"));
			}
		}
	};
	TokenStream::new()
}
*/

fn decl<T: Iterator<Item = TokenTree>>(previous: &TokenTree, input: &mut T) -> TokenStream {
	let group = match input.next() {
		Some(TokenTree::Group(group)) => group,
		Some(other) => return make_error("Expected ( .. )", Show(&other)),
		None => return make_error("Expected ( .. ) after `Decl`", Hide(previous)),
	};
	let mut group_stream = group.stream().into_iter();

	let decl_type = match group_stream.next() {
		Some(decl_type) => match decl_type.to_string().as_str() {
			"Const" | "Mut" | "Fn" => decl_type,
			_ => return make_error("Expected `Const | Mut | Fn`", Show(&decl_type)),
		},
		None => return make_error("Expected `Const | Mut | Fn` inside of ( .. )", Hide(&group)),
	};

	let name = match group_stream.next() {
		Some(name) if name.to_string().starts_with('"') => name,
		Some(other) => return make_error("Expected string literal", Show(&other)),
		None => {
			return make_error(
				&format!("Expected name as string literal after `{decl_type}`"),
				Hide(&decl_type),
			)
		}
	};

	let name_span = name.span();
	let mut output = TokenStream::new();
	// Stmt::Decl(Decl::Const("name".into(), todo!()))
	// Stmt::Decl
	output.append_all(quote_spanned!(name_span => Stmt::Decl));
	// // ( .. )
	output.append_all(make_group(Delimiter::Parenthesis, |ts| {
		// 	// Decl::(Const | Mut | Fn)
		ts.append_all(quote_spanned!(name_span => Decl::#decl_type));
		// 	// ( .. )
		ts.append_all(make_group(Delimiter::Parenthesis, |ts| {
			// "name".into(),
			ts.append(name.clone());
			ts.append_all(quote_spanned!(name_span => .into(),));

			// Fn => todo!()
			// __Expr__
			if &decl_type.to_string() == "Fn" {
				ts.append_all(quote_spanned!(name_span => todo!(),))
			}
			ts.append_all(expr(&name, &mut group_stream));
		}))
	}));
	output
}

fn expr<T: Iterator<Item = TokenTree>>(previous: &TokenTree, input: &mut T) -> TokenStream {
	let previous = match input.next() {
		Some(TokenTree::Ident(ident)) if &ident.to_string() == "Expr" => ident,
		Some(other) => return make_error("Expected `Expr`", Show(&other)),
		None => {
			return make_error(
				&format!("Expected `Expr` after `{previous}`"),
				Hide(previous),
			)
		}
	};

	match input.next() {
		Some(TokenTree::Group(group)) => (),
		Some(other) => return make_error("Expected ( .. )", Show(&other)),
		None => {
			return make_error(
				&format!("Expected ( .. ) after `{previous}`"),
				Hide(&previous),
			)
		}
	};

	quote!(todo!())

	// Some(TokenTree::Group(group)) => todo!(),
	// Some(other) => make_error(&format!("Expected ( .. ) after `{other}`"), Show(&other)),
}

fn make_group<F>(delimiter: Delimiter, f: F) -> TokenStream
where
	F: FnOnce(&mut TokenStream),
{
	Group::new(delimiter, {
		let mut ts = TokenStream::new();
		f(&mut ts);
		ts
	})
	.to_token_stream()
}

use Visibility::{Hide, Show};
enum Visibility<T> {
	Show(T),
	Hide(T),
}

impl<T> Visibility<T> {
	fn inner(self) -> T {
		match self {
			Visibility::Show(inner) | Visibility::Hide(inner) => inner,
		}
	}
}

fn make_error<S>(msg: &str, to_span: Visibility<&S>) -> TokenStream
where
	S: ToSpan + Display,
{
	let msg = match to_span {
		Show(to_span) => format!("{msg} got `{to_span}`"),
		Hide(_) => msg.into(),
	};
	quote_spanned! {
		to_span.inner().span().into() =>
		compile_error!(#msg)
	}
}

trait ToSpan {
	fn span(&self) -> Span;
}

impl ToSpan for Span {
	fn span(&self) -> Span {
		self.clone()
	}
}

impl ToSpan for TokenTree {
	fn span(&self) -> Span {
		self.span()
	}
}

impl ToSpan for Group {
	fn span(&self) -> Span {
		self.span()
	}
}

impl ToSpan for Ident {
	fn span(&self) -> Span {
		self.span()
	}
}

impl ToSpan for Literal {
	fn span(&self) -> Span {
		self.span()
	}
}

impl ToSpan for Punct {
	fn span(&self) -> Span {
		self.span()
	}
}
