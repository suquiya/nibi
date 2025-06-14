use super::token::BlockToken;

// 行番号などもう少し複雑な処理が必要になる場合に備えてstructにしておく
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
	pub start: usize,
}

impl From<usize> for Pos {
	fn from(value: usize) -> Self {
		Self { start: value }
	}
}

impl Pos {
	pub fn new(start: usize) -> Pos {
		Pos { start }
	}

	pub fn start(&self) -> &usize {
		&self.start
	}

	pub fn mut_start(&mut self) -> &mut usize {
		&mut self.start
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenNode {
	pub pos: Pos,
	pub token: BlockToken,
}

impl TokenNode {
	pub fn new<T: Into<Pos>>(pos: T, token: BlockToken) -> TokenNode {
		TokenNode {
			pos: pos.into(),
			token,
		}
	}
}

impl TokenNode {
	pub fn get_string_value(&self) -> Option<&str> {
		self.token.get_string_value()
	}
}
