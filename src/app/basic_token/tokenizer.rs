use crate::app::basic_token::token::BasicToken;

#[derive(Debug)]
/// Tokenizer for the ingot format.
pub struct BasicTokenizer {
	chars: Vec<char>,
	/// Current position of cursor in chars.
	pub pos: usize,
}

const SYMBOL_CHARS: &str = "{}[]()<>,;: \t\n\r\"'/";

impl BasicTokenizer {
	/// constructor
	pub fn new(chars: Vec<char>) -> Self {
		Self { chars, pos: 0 }
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

	/// returns true if the character is a symbol character
	pub fn is_symbol_char(c: char) -> bool {
		SYMBOL_CHARS.contains(c)
	}

	fn tokenize_new_line_r(&mut self) -> BasicToken {
		let next = self.peek_next_char();
		match next {
			Some('\n') => {
				self.pos_next();
				BasicToken::newline("\r\n".to_string())
			}
			_ => BasicToken::newline("\r".to_string()),
		}
	}

	fn tokenize_whitespaces(&mut self, first_char: char) -> BasicToken {
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

		BasicToken::whitespaces(result)
	}

	fn tokenize_after_slash(&mut self) -> BasicToken {
		let next = self.peek_next_char();
		match next {
			Some('/') => {
				self.pos_next();
				BasicToken::line_comment_begin()
			}
			Some('*') => {
				self.pos_next();
				BasicToken::block_comment_begin()
			}
			_ => BasicToken::literal('/'.to_string()),
		}
	}

	fn tokenize_after_asterisk(&mut self) -> BasicToken {
		let next = self.peek_next_char();
		match next {
			Some('/') => {
				self.pos_next();
				BasicToken::block_comment_end()
			}
			_ => BasicToken::literal('*'.to_string()),
		}
	}

	fn tokenize_string(&mut self, first_char: char) -> BasicToken {
		let mut result = String::from(first_char);
		if first_char == '\\'
			&& let Some(nc) = self.peek_next_char()
		{
			result.push(*nc);
			self.pos_next();
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
		BasicToken::literal(result)
	}
	/// Returns the next raw token from chars.
	pub fn next(&mut self) -> BasicToken {
		let next_char = self.next_char();
		let token: BasicToken = if let Some(c) = next_char {
			match c {
				'\'' => BasicToken::single_quote(),
				'"' => BasicToken::double_quote(),
				';' => BasicToken::semicolon(),
				',' => BasicToken::comma(),
				':' => BasicToken::colon(),
				'\n' => BasicToken::newline("\n".to_string()),
				'\r' => self.tokenize_new_line_r(),
				' ' => self.tokenize_whitespaces(' '),
				'\t' => self.tokenize_whitespaces('\t'),
				'[' | '{' | '(' | '<' => BasicToken::bracket_begin(c.into()),
				']' | '}' | ')' | '>' => BasicToken::bracket_end(c.into()),
				'/' => self.tokenize_after_slash(),
				'*' => self.tokenize_after_asterisk(),
				_ => self.tokenize_string(c),
			}
		} else {
			BasicToken::eos()
		};

		token
	}
	/// Returns the rest of the chars after the current position.
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

	use super::*;

	#[test]
	fn test_raw_tokenize_basic() {
		let mut tokenizer = BasicTokenizer::new("aaa:bbb".chars().collect());
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::literal("aaa".to_string()));
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::colon());
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::literal("bbb".to_string()));
	}

	#[test]
	fn test_raw_tokenize_bracket() {
		let mut tokenizer = BasicTokenizer::new("aaa: {bbb: ccc}".chars().collect());
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::literal("aaa".to_string()));
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::colon());
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::bracket_begin("{".into()));
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::literal("bbb".to_string()));
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::colon());
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::literal("ccc".to_string()));
		let token = tokenizer.next();
		assert_eq!(token, BasicToken::bracket_end("}".into()));
	}
}
