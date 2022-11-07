use super::*;

#[test]
fn lex_keywords() {
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
		]
	)
}

#[test]
fn strings() {
	let source = r#" "str€ng" "#;
	let lexer = Lexer::new(source);
	let tokens: Vec<Spanned<Token>> = lexer.map(|token| token.unwrap()).collect();
	assert_eq!(
		tokens,
		vec![Spanned(
			Token::literal(TokenType::String, Literal::String("str€ng".into())),
			1..10
		)]
	)
}

#[test]
fn number() {
	let source = r#" 9 0.23042 "#;
	let lexer = Lexer::new(source);
	let tokens: Vec<Spanned<Token>> = lexer.map(|token| token.unwrap()).collect();
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
			)
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
			LexError::InvalidChar {
				msg: "Found invalid character `$`".into(),
				line: 1,
				col: 1
			},
			LexError::InvalidChar {
				msg: "Found invalid character `§`".into(),
				line: 1,
				col: 2
			}
		]
	)
}
