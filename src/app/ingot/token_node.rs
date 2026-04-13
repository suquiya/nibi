use super::token::BlockToken;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the position of a token in the source code.
/// For more complex position handling, `Pos` is defined as a struct.
pub struct Pos {
	/// The start position of the token in the source code.
	pub start: usize,
}

impl From<usize> for Pos {
	fn from(value: usize) -> Self {
		Self { start: value }
	}
}

impl Pos {
	/// Creates a new `Pos` instance with the given start position.
	pub fn new(start: usize) -> Pos {
		Pos { start }
	}
	/// Returns the start position of the token in the source code.
	pub fn start(&self) -> &usize {
		&self.start
	}
	/// Returns a mutable reference to the start position of the token in the source code.
	pub fn mut_start(&mut self) -> &mut usize {
		&mut self.start
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// TokenNode struct represents a token node in the AST.
/// It contains the position of the token in the source code and the token itself.
pub struct TokenNode {
	/// The position of the token in the source code.
	pub pos: Pos,
	/// The token itself.
	pub token: BlockToken,
}

impl TokenNode {
	/// Creates a new `TokenNode` instance with the given position and token.
	pub fn new<T: Into<Pos>>(pos: T, token: BlockToken) -> TokenNode {
		TokenNode {
			pos: pos.into(),
			token,
		}
	}
}

impl TokenNode {
	/// Returns the string value of the token, if it has one.
	pub fn get_string_value(&self) -> Option<&str> {
		self.token.get_string_value()
	}
}
