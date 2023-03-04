```rym
const arr: [1, 2, 3] = [1, 2, 3]
const item: 1 = arr[0]

mod core {
	type Bool = true | false;

	type Int;
	type Isize;
	type Uint;
	type Usize;

	type Array[Item, Len: Usize] {
		len: Len,
	}

	type List[Item, Len: Usize, Cap: Usize] = {
		mut buf: Array[Item, Cap],
		mut len: Len,
	};

	impl[Item] List[Item] {
		func new(..buf: Array[Item, @len]) -> List[Item, len, len] {
			Self { buf, len }
		}

		func push(mut self, value: Item) {
			if self.len == self.buf.len then {
				self.buf = [..self.buf, value]
			} else {
				self.buf.insert(self.len + 1, )
			}
			self.len += 1;
		}
	}

	test {
		mut nums = List.new(1, 2, 3, 4);
		nums.push(5);
	}

	type Option a = Some a | None;
	use Option.{Some, None};

	test {
		const maybe_num = Some 2;

		match maybe_num with
			| Some num => print(f"got {num}"),
			| None => print(f"got nothing");
	}
}

{
	pub type Item =
		| Module {
			name: Spanned[String],
			items: List[Spanned[Item]] = List.new(),
		}
		| Func {
			name: Spanned[String],
			params: List[Spanned[String]] = List.new(),
			rhs: Option[Spanned[Expr]] = None,
		}
		| Var {
			mutable: bool,
			name: Spanned[String],
			rhs: Option[Spanned[Expr]],
		};

	pub type Item = {
		type Module = {
			name: Spanned[String],
			items: List[Spanned[Item]] = List.new(),
		};
		type Func = {
			name: Spanned[String],
			params: List[Spanned[String]] = List.new(),
			rhs: Option[Spanned[Expr]] = None,
		};
		type Var = {
			mutable: bool,
			name: Spanned[String],
			rhs: Option[Spanned[Expr]],
		};
		Module | Func | Var
	}

	pub enum Item {
		Module {
			name: Spanned[String],
			items: List[Spanned[Item]] = List.new(),
		},
		Func {
			name: Spanned[String],
			params: List[Spanned[String]] = List.new(),
			rhs: Option[Spanned[Expr]] = None,
		},
		Var {
			mutable: bool,
			name: Spanned[String],
			rhs: Option[Spanned[Expr]],
		},
	}

	pub enum Item {
		Module {
			name: Spanned<String>,
			items: List<Spanned<Item>> = List.new(),
		},
		Func {
			name: Spanned<String>,
			params: List<Spanned<String>> = List.new(),
			rhs: Option<Spanned<Expr>> = None,
		},
		Var {
			mutable: bool,
			name: Spanned<String>,
			rhs: Option<Spanned<Expr>>,
		},
	}

	const module = Item.Module { name: ("test_module", 0..89) };
	const module = Item.Module {
		name: ("test_module", 5..15),
		items: List.new(
			(.Func { name: ("t2", 20..21) }, 0..0),
			(.Func { name: ("t3", 0..0) }, 0..0),
		)
	};
	const function = Item.Func { name: ("testing", 0..0) };
	const variable = Item.Var { mutable: true, name: ("test", 0..0) };
}
use std.ops.Add;

type Uint

trait Add[Rhs = Self] {
	type Output = Self

	func add(self, other: Output) Self
}

impl Add for Uint {
	func add(self, other: Uint) Uint {
		panic("compiler internal")
	}
}
```
