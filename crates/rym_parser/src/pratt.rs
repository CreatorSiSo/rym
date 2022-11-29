#![cfg(test)]
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
