mod token_tree;

pub use token_tree::TokenTree;

/// A `TokenStream` is an abstract sequence of tokens, organized into [`TokenTree`]s.
#[derive(Debug, PartialEq)]
pub struct TokenStream {
	stream: Vec<TokenTree>,
}

impl TokenStream {
	pub const fn new(stream: Vec<TokenTree>) -> Self {
		Self { stream }
	}
}
