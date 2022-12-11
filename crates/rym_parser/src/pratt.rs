enum S {
	Cons(char, Vec<S>),
}

impl core::fmt::Display for S {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			S::Cons(head, rest) => {
				if rest.is_empty() {
					write!(f, "{}", head)
				} else {
					write!(f, "({}", head)?;
					for s in rest {
						write!(f, " {}", s)?
					}
					write!(f, ")")
				}
			}
		}
	}
}

struct Lexer {
	tokens: Vec<char>,
}

impl Lexer {
	fn new(input: &str) -> Lexer {
		let mut tokens = input.chars().filter(|it| !it.is_ascii_whitespace()).collect::<Vec<_>>();
		tokens.reverse();
		Lexer { tokens }
	}

	fn next(&mut self) -> Option<char> {
		self.tokens.pop()
	}

	fn peek(&mut self) -> Option<char> {
		self.tokens.last().copied()
	}
}

fn expr(input: &str) -> S {
	let mut lexer = Lexer::new(input);
	expr_bp(&mut lexer, 0).unwrap()
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Option<S> {
	let mut lhs = None;

	loop {
		let token = lexer.peek();
		let (token, r_bp) = match binding_power(token, lhs.is_none()) {
			Some((t, (l_bp, r_bp))) if min_bp <= l_bp => (t, r_bp),
			_ => return lhs,
		};

		lexer.next();

		let rhs = expr_bp(lexer, r_bp);
		if token == '(' {
			assert_eq!(lexer.next(), Some(')'));
			lhs = rhs;
			continue;
		}

		let mut args = Vec::new();
		args.extend(lhs);
		args.extend(rhs);
		lhs = Some(S::Cons(token, args));
	}
}

fn binding_power(op: Option<char>, prefix: bool) -> Option<(char, (u8, u8))> {
	let op = op?;
	let res = match op {
		'0'..='9' | 'a'..='z' | 'A'..='Z' => (99, 100),
		'(' => (99, 0),
		'=' => (2, 1),
		'+' | '-' if prefix => (99, 9),
		'+' | '-' => (5, 6),
		'*' | '/' => (7, 8),
		'!' => (11, 100),
		'.' => (14, 13),
		_ => return None,
	};
	Some((op, res))
}

#[test]
fn tests() {
	let s = expr("1");
	assert_eq!(s.to_string(), "1");

	let s = expr("1 + 2 * 3");
	assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

	let s = expr("a + b * c * d + e");
	assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

	let s = expr("f . g . h");
	assert_eq!(s.to_string(), "(. f (. g h))");

	let s = expr(" 1 + 2 + f . g . h * 3 * 4");
	assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");

	let s = expr("--1 * 2");
	assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

	let s = expr("--f . g");
	assert_eq!(s.to_string(), "(- (- (. f g)))");

	let s = expr("-9!");
	assert_eq!(s.to_string(), "(- (! 9))");

	let s = expr("f . g !");
	assert_eq!(s.to_string(), "(! (. f g))");

	let s = expr("(((0)))");
	assert_eq!(s.to_string(), "0");

	let s = expr("(1 + 2) * 3");
	assert_eq!(s.to_string(), "(* (+ 1 2) 3)");

	let s = expr("1 + (2 * 3)");
	assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
}

#[test]
fn pratt() {
	enum Token {
		Number(f64),
		Plus,
		Minus,
		Star,
		Slash,
		LParen,
		RParen,
		EOF,
	}

	struct Parser {
		tokens: Vec<Token>,
		current: usize,
	}

	#[derive(Debug, PartialEq)]
	enum Expr {
		Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
		Unary { op: UnaryOp, right: Box<Expr> },
		Number(f64),
	}

	impl Parser {
		fn parse_expression(&mut self, precedence: i32) -> Expr {
			let mut left = self.parse_prefix();

			while precedence < self.get_token_precedence() {
				left = self.parse_infix(left);
			}

			left
		}

		fn parse_prefix(&mut self) -> Expr {
			match self.tokens[self.current] {
				Token::Number(num) => {
					self.current += 1;
					Expr::Number(num)
				}
				Token::Minus => {
					self.current += 1;
					Expr::Unary { op: UnaryOp::Neg, right: Box::new(self.parse_expression(100)) }
				}
				Token::LParen => {
					self.current += 1;
					let node = self.parse_expression(0);
					self.current += 1;
					node
				}
				_ => {
					panic!()
				}
			}
		}

		fn parse_infix(&mut self, left: Expr) -> Expr {
			let precedence = self.get_token_precedence();
			let token = &self.tokens[self.current];
			self.current += 1;

			match token {
				Token::Plus => Expr::Binary {
					op: BinaryOp::Add,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Minus => Expr::Binary {
					op: BinaryOp::Sub,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Star => Expr::Binary {
					op: BinaryOp::Mul,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				Token::Slash => Expr::Binary {
					op: BinaryOp::Div,
					left: Box::new(left),
					right: Box::new(self.parse_expression(precedence)),
				},
				_ => {
					panic!()
				}
			}
		}

		fn get_token_precedence(&mut self) -> i32 {
			let token = &self.tokens[self.current];

			match token {
				Token::Plus | Token::Minus => 2,
				Token::Star | Token::Slash => 3,
				Token::LParen | Token::RParen => 1,
				_ => -1,
			}
		}
	}

	let tokens = vec![
		Token::Number(2.0),
		Token::Plus,
		Token::Minus,
		Token::Number(3.0),
		Token::Star,
		Token::Number(4.0),
		Token::EOF,
	];

	let mut parser = Parser { tokens, current: 0 };
	let ast = parser.parse_expression(0);

	assert_eq!(
		ast,
		Expr::Binary {
			op: BinaryOp::Add,
			left: Box::new(Expr::Number(2.0)),
			right: Box::new(Expr::Binary {
				op: BinaryOp::Mul,
				left: Box::new(Expr::Unary { op: UnaryOp::Neg, right: Box::new(Expr::Number(3.0)) }),
				right: Box::new(Expr::Number(4.0))
			})
		}
	);

	let tokens = vec![Token::Number(2.0), Token::EOF];

	let mut parser = Parser { tokens, current: 0 };
	let ast = parser.parse_expression(0);

	assert_eq!(ast, Expr::Number(2.0));
}
