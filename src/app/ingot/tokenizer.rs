use super::token::{
	Bracket, BracketRole, BracketType, CommentMark, Quote, RawToken, RawTokenData, Sep,
};

#[derive(Debug)]
pub struct IngotTokenizer {
	chars: Vec<char>,
	pub pos: usize,
}

const SYMBOL_CHARS: &str = "{}[]()<>,;: \t\n\r\"'/";

impl IngotTokenizer {
	/// constructor
	pub fn new(chars: Vec<char>) -> IngotTokenizer {
		IngotTokenizer { chars, pos: 0 }
	}

	/// returns next char
	pub fn next_char(&mut self) -> Option<char> {
		let result = self.peek_next_char().copied();
		self.pos += 1;
		result
	}

	/// returns next char without move position
	pub fn peek_next_char(&self) -> Option<&char> {
		self.chars.get(self.pos)
	}

	/// backs position
	pub fn pos_back(&mut self) {
		self.pos -= 1;
	}

	/// moves position
	pub fn pos_next(&mut self) {
		self.pos += 1;
	}

	pub fn is_symbol_char(c: char) -> bool {
		SYMBOL_CHARS.contains(c)
	}

	fn tokenize_new_line(&mut self) -> RawToken {
		let next = self.peek_next_char();
		match next {
			Some('\n') => {
				self.pos_next();
				RawToken::Sep(Sep::NewLine)
			}
			Some(_c) => RawToken::Sep(Sep::NewLine),
			None => RawToken::Sep(Sep::NewLine),
		}
	}

	fn tokenize_whitespaces(&mut self, first_char: char) -> RawToken {
		let mut result = String::from(first_char);
		loop {
			let next = self.peek_next_char();
			match next {
				Some(c) => {
					let c = *c;
					if c == ' ' || c == '\t' {
						result.push(c);
						self.pos_next();
					} else {
						break;
					}
				}
				_ => break,
			}
		}

		RawToken::Sep(Sep::WhiteSpaces(result))
	}

	fn tokenize_after_slash(&mut self) -> RawToken {
		let next = self.peek_next_char();
		match next {
			Some('/') => {
				self.pos_next();
				RawToken::Comment(CommentMark::LineBegin)
			}
			Some('*') => {
				self.pos_next();
				RawToken::Comment(CommentMark::BlockBegin)
			}
			_ => RawToken::SimpleString('/'.to_string()),
		}
	}

	fn tokenize_after_asterisk(&mut self) -> RawToken {
		let next = self.peek_next_char();
		match next {
			Some('/') => {
				self.pos_next();
				RawToken::Comment(CommentMark::BlockEnd)
			}
			_ => RawToken::SimpleString('*'.to_string()),
		}
	}

	fn tokenize_string(&mut self, first_char: char) -> RawToken {
		let mut result = String::from(first_char);
		if first_char == '\\' {
			if let Some(nc) = self.peek_next_char() {
				result.push(*nc);
				self.pos_next();
			}
		}

		loop {
			let next = self.peek_next_char();
			match next {
				Some(c) => {
					let c = *c;
					if SYMBOL_CHARS.contains(c) {
						break;
					} else if c == '\\' {
						result.push(c);
						self.pos_next();
						match self.peek_next_char() {
							Some(nc) => {
								result.push(*nc);
								self.pos_next();
							}
							_ => break,
						}
					} else if c == '*' {
						match self.peek_next_char() {
							Some('/') => {
								break;
							}
							Some(nc) => {
								result.push(c);
								result.push(*nc);
								self.pos += 2;
							}
							None => {
								break;
							}
						}
					} else {
						result.push(c);
						self.pos_next();
					}
				}
				_ => break,
			}
		}
		RawToken::SimpleString(result)
	}

	pub fn next_raw_token(&mut self) -> RawTokenData {
		let pos = self.pos;
		let next_char = self.next_char();
		let token: RawToken = if let Some(c) = next_char {
			match c {
				'\'' => RawToken::Quote(Quote::Single),
				'"' => RawToken::Quote(Quote::Double),
				',' => RawToken::Sep(Sep::Comma),
				':' => RawToken::Sep(Sep::Colon),
				'\n' => RawToken::Sep(Sep::NewLine),
				'\r' => self.tokenize_new_line(),
				' ' => self.tokenize_whitespaces(' '),
				'\t' => self.tokenize_whitespaces('\t'),
				'[' => RawToken::Bracket(Bracket::new(BracketRole::Start, BracketType::Square)),
				']' => RawToken::Bracket(Bracket::new(BracketRole::End, BracketType::Square)),
				'{' => RawToken::Bracket(Bracket::new(BracketRole::Start, BracketType::Curly)),
				'}' => RawToken::Bracket(Bracket::new(BracketRole::End, BracketType::Curly)),
				'(' => RawToken::Bracket(Bracket::new(BracketRole::Start, BracketType::Normal)),
				')' => RawToken::Bracket(Bracket::new(BracketRole::End, BracketType::Normal)),
				'<' => RawToken::Bracket(Bracket::new(BracketRole::Start, BracketType::Angle)),
				'>' => RawToken::Bracket(Bracket::new(BracketRole::End, BracketType::Angle)),
				'/' => self.tokenize_after_slash(),
				'*' => self.tokenize_after_asterisk(),
				_ => self.tokenize_string(c),
			}
		} else {
			RawToken::Eos
		};

		(pos, token)
	}

	pub fn get_rest_all(&mut self) -> (usize, Vec<char>) {
		if self.chars.len() > self.pos {
			(self.pos, self.chars.split_off(self.pos))
		} else {
			(self.pos, Vec::new())
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::app::ingot::token::BracketType;

	use super::*;

	#[test]
	fn test_raw_tokenize_basic() {
		let mut tokenizer = IngotTokenizer::new("aaa:bbb".chars().collect());
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 0);
		assert_eq!(token, RawToken::SimpleString("aaa".to_string()));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 3);
		assert_eq!(token, RawToken::Sep(Sep::Colon));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 4);
		assert_eq!(token, RawToken::SimpleString("bbb".to_string()));
	}

	#[test]
	fn test_raw_tokenize_bracket() {
		let mut tokenizer = IngotTokenizer::new("aaa: {bbb: ccc}".chars().collect());
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 0);
		assert_eq!(token, RawToken::SimpleString("aaa".to_string()));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 3);
		assert_eq!(token, RawToken::Sep(Sep::Colon));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 4);
		assert_eq!(token, RawToken::Sep(Sep::WhiteSpaces(" ".to_string())));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 5);
		assert_eq!(
			token,
			RawToken::Bracket(Bracket::new(BracketRole::Start, BracketType::Curly))
		);
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 6);
		assert_eq!(token, RawToken::SimpleString("bbb".to_string()));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 9);
		assert_eq!(token, RawToken::Sep(Sep::Colon));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 10);
		assert_eq!(token, RawToken::Sep(Sep::WhiteSpaces(" ".to_string())));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 11);
		assert_eq!(token, RawToken::SimpleString("ccc".to_string()));
		let (pos, token) = tokenizer.next_raw_token();
		assert_eq!(pos, 14);
		assert_eq!(
			token,
			RawToken::Bracket(Bracket::new(BracketRole::End, BracketType::Curly))
		);
	}
}
