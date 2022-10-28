extern crate proc_macro;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenTree};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

#[proc_macro]
pub fn make_ast(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = proc_macro2::TokenStream::from(input).into_iter();
	let mut output = proc_macro2::TokenStream::new();

	// let ___v: Vec<Stmt>;
	output.append_all(quote!(let ___v: Vec<Stmt>;));

	// __v = vec![ /* generated ast */ ];
	output.append(Ident::new("___v", Span::call_site()));
	output.append(Punct::new('=', Spacing::Alone));
	output.append(Ident::new("vec", Span::call_site()));
	output.append(Punct::new('!', Spacing::Joint));
	output.append(Group::new(Delimiter::Bracket, stmts(input)));
	output.append(Punct::new(';', Spacing::Joint));

	// __v
	output.append(Ident::new("___v", Span::call_site()));

	// { .. }
	Group::new(Delimiter::Brace, output)
		.into_token_stream()
		.into()
}

fn stmts<T: Iterator<Item = TokenTree>>(mut input: T) -> proc_macro2::TokenStream {
	let mut stmts: Vec<proc_macro2::TokenStream> = Vec::new();
	let mut push = |stream: proc_macro2::TokenStream| stmts.push(stream);

	while let Some(token) = input.next() {
		match token {
			TokenTree::Group(group) => {
				let mut tokens = group.stream().into_iter();
				match tokens.next() {
					Some(tt) => match tt.to_string().as_str() {
						"Expr" => push(expr(&mut tokens)),
						typ @ ("Const" | "Mut" | "Fn") => push(decl(typ, &mut tokens)),
						str => push(error(
							&tt,
							&format!("Expected `Const | Mut | Fn | Expr` got `{str}`"),
						)),
					},
					None => push(quote!(Stmt::Empty)),
				}
			}
			token => push(error(&token, &format!("Expected `( .. )` got {token}"))),
		};

		comma(&mut input);
	}

	let mut output = proc_macro2::TokenStream::new();
	output.append_separated(stmts.into_iter(), Punct::new(',', Spacing::Alone));
	output
}

fn comma<T: Iterator<Item = TokenTree>>(tokens: &mut T) {
	match tokens.next() {
		Some(token) => match token {
			TokenTree::Punct(punct) => match punct.as_char() {
				',' => {}
				char => panic!("Expected `,` got `{char}`"),
			},
			tt => panic!("Expected `,` got `{tt}`"),
		},
		None => {
			if let Some(tt) = tokens.next() {
				panic!("Expected `,` got {tt}")
			}
		}
	}
}

fn decl<T: Iterator<Item = TokenTree>>(typ: &str, tokens: &mut T) -> proc_macro2::TokenStream {
	match typ {
		"Const" => match tokens.next().expect("Expected name after `Decl Const`") {
			TokenTree::Literal(lit) => {
				let name = lit.to_string();
				if name.starts_with('"') && name.ends_with('"') {
					let mut source = String::from("Stmt::Decl(Decl::Const(");
					source += &name;
					source += ".into(), todo!()))";
					source
				} else {
					panic!("Expected string literal got `{lit}`")
				}
			}
			tt => panic!("Expected string literal got `{tt}`"),
		},
		"Mut" => stringify!(Stmt::Decl(Decl::Mut("__todo__".into(), todo!()))).into(),
		"Fn" => stringify!(Stmt::Decl(Decl::Fn("__todo__".into(), vec![], todo!()))).into(),
		_ => unreachable!(),
	};
	proc_macro2::TokenStream::new()
}

fn expr<T: Iterator<Item = TokenTree>>(tokens: &mut T) -> proc_macro2::TokenStream {
	proc_macro2::TokenStream::new()
}

fn error<S: ToSpan>(to_span: &S, msg: &str) -> proc_macro2::TokenStream {
	quote_spanned! {
		to_span.span().into() =>
		compile_error!(#msg)
	}
}

trait ToSpan {
	fn span(&self) -> Span;
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
