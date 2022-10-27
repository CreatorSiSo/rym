extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn statements(tokens: TokenStream) -> TokenStream {
	let mut source = String::from("vec![");
	let mut tokens = tokens.into_iter();
	let mut push_stmt = |str: &str| {
		source += &str;
		source += ", ";
	};

	while let Some(token) = tokens.next() {
		match token {
			TokenTree::Group(group) => {
				let mut tokens = group.stream().into_iter();
				match tokens.next() {
					Some(tt) => match tt {
						TokenTree::Ident(ident) => match ident.to_string().as_str() {
							"Expr" => push_stmt(&expr(&mut tokens)),
							typ @ ("Const" | "Mut" | "Fn") => push_stmt(&decl(typ, &mut tokens)),
							str => panic!("Expected `Const | Mut | Fn | Expr` got `{str}`"),
						},
						tt => panic!("Expected identifier got `{tt}`"),
					},
					None => push_stmt("Stmt::Empty"),
				}
			}
			_ => panic!("Expected Group"),
		}

		comma(&mut tokens);
	}

	source += "]";
	source.parse().unwrap()
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

fn expr<T: Iterator<Item = TokenTree>>(tokens: &mut T) -> String {
	"todo!()".into()
}

fn decl<T: Iterator<Item = TokenTree>>(typ: &str, tokens: &mut T) -> String {
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
	}
}
