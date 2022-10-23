use ast::{Literal, Spanned, SpannedToken, Token, TokenType};
use lex::{Lexer, LexerError};

#[test]
fn file() {
	let source = include_str!("./lex.rym");
	let mut lexer = Lexer::new(source);
	loop {
		match lexer.next_token() {
			Ok(token) => {
				println!("{token:?}");
				if token.0.typ == TokenType::Eof {
					break;
				}
			}
			Err(err) => println!("{err:?}"),
		}
	}
}

#[test]
fn keywords() {
	let source = "if else for while loop return break false true fn const mut struct self";
	let lexer = Lexer::new(source);
	let tokens: Vec<TokenType> = lexer.map(|token| token.unwrap().0.typ).collect();
	assert_eq!(
		tokens,
		vec![
			TokenType::If,
			TokenType::Else,
			TokenType::For,
			TokenType::While,
			TokenType::Loop,
			TokenType::Return,
			TokenType::Break,
			TokenType::False,
			TokenType::True,
			TokenType::Fn,
			TokenType::Const,
			TokenType::Mut,
			TokenType::Struct,
			TokenType::Self_,
			TokenType::Eof
		]
	)
}

#[test]
fn operators() {
	let source = "- + / * ! != = == > >= < <=	&& ||";
	let lexer = Lexer::new(source);
	let tokens: Vec<TokenType> = lexer.map(|token| token.unwrap().0.typ).collect();
	assert_eq!(
		tokens,
		vec![
			TokenType::Minus,
			TokenType::Plus,
			TokenType::Slash,
			TokenType::Star,
			TokenType::Bang,
			TokenType::BangEqual,
			TokenType::Equal,
			TokenType::EqualEqual,
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
			TokenType::DoubleAmpersand,
			TokenType::DoublePipe,
			TokenType::Eof
		]
	)
}

#[test]
fn special_chars() {
	let source = ". , ; ( ) { }";
	let lexer = Lexer::new(source);
	let tokens: Vec<TokenType> = lexer.map(|token| token.unwrap().0.typ).collect();
	assert_eq!(
		tokens,
		vec![
			TokenType::Dot,
			TokenType::Comma,
			TokenType::Semicolon,
			TokenType::LeftParen,
			TokenType::RightParen,
			TokenType::LeftBrace,
			TokenType::RightBrace,
			TokenType::Eof
		]
	)
}

#[test]
fn strings() {
	let source = r#" "str€ng" "#;
	let lexer = Lexer::new(source);
	let tokens: Vec<SpannedToken> = lexer.map(|token| token.unwrap()).collect();
	assert_eq!(
		tokens,
		vec![
			Spanned(
				Token::literal(TokenType::String, Literal::String("str€ng".into())),
				1..10
			),
			Spanned(Token::new(TokenType::Eof), 11..12)
		]
	)
}

#[test]
fn number() {
	let source = r#" 9 0.23042 "#;
	let lexer = Lexer::new(source);
	let tokens: Vec<SpannedToken> = lexer.map(|token| token.unwrap()).collect();
	assert_eq!(
		tokens,
		vec![
			Spanned(
				Token::literal(TokenType::Number, Literal::Number(9.0)),
				1..1
			),
			Spanned(
				Token::literal(TokenType::Number, Literal::Number(0.23042)),
				3..9
			),
			Spanned(Token::new(TokenType::Eof), 10..11)
		]
	)
}

#[test]
fn lexer_errors() {
	let source = r#"$§ "$§" 00.9 9495346598934856389453945658934653765898"#;
	let (tokens, errors) = Lexer::lex(source);
	dbg!(tokens, &errors);
	assert_eq!(
		errors,
		vec![
			LexerError::InvalidChar {
				msg: "Found invalid character `$`".into(),
				line: 1,
				col: 1
			},
			LexerError::InvalidChar {
				msg: "Found invalid character `§`".into(),
				line: 1,
				col: 2
			}
		]
	)
}
