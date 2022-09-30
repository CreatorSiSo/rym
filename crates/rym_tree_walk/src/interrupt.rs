use rym_ast::Literal;

pub enum Inter<'src> {
	Break(Literal<'src>),
	Continue,
	None(Literal<'src>),
}

// impl<'src> Inter<'src> {
// 	pub fn is_break(&self) -> bool {
// 		match self {
// 			Self::Break(_) => true,
// 			_ => false,
// 		}
// 	}
// }

impl<'src> Into<Literal<'src>> for Inter<'src> {
	fn into(self) -> Literal<'src> {
		match self {
			Inter::Break(lit) | Inter::None(lit) => lit,
			_ => Literal::Unit
		}
	}
}
