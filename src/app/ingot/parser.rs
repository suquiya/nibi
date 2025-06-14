use std::io::Read;

use jiff::Timestamp;

use crate::app::{
	fs::io::read_all_from_reader,
	ingot::ingot::{RKeyList, To},
};

use super::{
	Ingot,
	error::ParseError,
	token::{BlockToken, Bracket, BracketRole, CommentMark, Quote, RawToken, RawTokenData, Sep},
	token_node::TokenNode,
	tokenizer::IngotTokenizer,
};

#[derive(Debug)]
pub struct IngotParser {}

impl IngotParser {
	pub fn set_from_key_value(
		&mut self,
		result: &mut Ingot,
		key: String,
		value: Box<Option<TokenNode>>,
	) {
		if let Some(v) = *value {
			let token = v.token;
			match key.as_str() {
				"tags" | "tag" => {
					result.tags = RKeyList::from(token);
				}
				"categories" | "category" => {
					result.categories = RKeyList::from(token);
				}
				"type" | "to" => {
					let val = token.get_string_value_or_empty();
					result.to = To::from(val.as_str().trim());
				}
				"updated" | "modified" => {
					match token
						.get_string_value_or_empty()
						.trim()
						.parse::<Timestamp>()
					{
						Ok(val) => result.updated = val,
						Err(_e) => (),
					}
				}
				"created" | "published" => {
					match token
						.get_string_value_or_empty()
						.trim()
						.parse::<Timestamp>()
					{
						Ok(val) => result.published = val,
						Err(_e) => (),
					}
				}
				"ingot_id" | "id" => match token.get_string_value_or_empty().trim().parse::<usize>() {
					Ok(val) => result.id = val,
					Err(_e) => (),
				},
				"url_path_name" | "post_url_name" | "page_url_name" | "pname" => {
					let val = token.get_string_value_or_empty();
					if !val.is_empty() {
						result.pname = val;
					}
				}
				_ => (),
			}
		}
	}
	pub fn parse<R: Read>(mut reader: R) -> Result<Ingot, ParseError> {
		let mut buffer = read_all_from_reader(reader).map_err(ParseError::IO)?;

		let mut result = Ingot::default();
		let mut tokenizer = IngotTokenizer::new(buffer);

		// フロントマターを分離する
		let mut front_matter_tokens: Vec<RawTokenData> = Vec::new();
		let mut prev_nl = false;
		loop {
			let (pos, token) = tokenizer.next_raw_token();
			match token {
				RawToken::Eos => break,
				RawToken::Sep(Sep::NewLine) => {
					if prev_nl {
						break;
					}
					front_matter_tokens.push((pos, token));
					prev_nl = true;
				}
				_ => {
					front_matter_tokens.push((pos, token));
					prev_nl = false;
				}
			}
		}

		buffer = tokenizer.get_rest_all();

		// println!("front_matter: {:#?}", front_matter_tokens);

		let mut parser = IngotMatterTokenParser::new(front_matter_tokens);
		loop {
			let node = parser.next_token_node();
			match node {
				Some(t_node) => if let BlockToken::KeyValue(key, value) = t_node.token {},
				_ => break,
			}
		}

		//println!("buffer: {:#?}", buffer);

		Ok(result)
	}
}

pub struct IngotMatterTokenParser {
	pub raw_tokens: Vec<RawTokenData>,
	pub pos: usize,
}

impl IngotMatterTokenParser {
	pub fn new(raw_tokens: Vec<RawTokenData>) -> IngotMatterTokenParser {
		IngotMatterTokenParser { raw_tokens, pos: 0 }
	}

	pub fn peek_next_token(&self) -> Option<&RawTokenData> {
		self.raw_tokens.get(self.pos)
	}

	pub fn next_token(&mut self) -> Option<RawTokenData> {
		let result = self.peek_next_token().cloned();
		self.pos += 1;
		result
	}

	pub fn pos_next(&mut self) {
		self.pos += 1;
	}

	pub fn pos_back(&mut self) {
		self.pos -= 1;
	}

	pub fn seek_and_get_string_until(&mut self, stop_tokens: Vec<RawToken>) -> String {
		let mut result = String::new();
		loop {
			let next_token = self.peek_next_token();
			if let Some((_, token)) = next_token {
				if stop_tokens.contains(token) || token == &RawToken::Eos {
					break;
				}
				let s = token.get_as_string();
				result.push_str(&s);
				self.pos_next();
			} else {
				break;
			}
		}
		result
	}

	pub fn parse_quoted_block(&mut self, pos: usize, quote: Quote) -> TokenNode {
		let content = self.seek_and_get_string_until(vec![
			RawToken::Quote(quote.clone()),
			RawToken::Sep(Sep::NewLine),
		]);

		let mut next_token = self.peek_next_token();

		if let Some((_, RawToken::Quote(q))) = next_token {
			if q == &quote {
				self.pos_next();
				next_token = self.peek_next_token();
			}
		}

		match next_token {
			Some((_, RawToken::Sep(Sep::NewLine))) => {
				self.pos_next();
			}
			Some((_, RawToken::Sep(Sep::Colon))) => {
				self.pos_next();
				return self.parse_key_value(pos, content);
			}
			_ => (),
		}
		TokenNode::new(pos, BlockToken::QuotedString(quote, content))
	}

	fn parse_key_value(&mut self, pos: usize, key: String) -> TokenNode {
		let mut value = self.next_token_node();
		// コメントをスキップ
		if let Some(TokenNode {
			pos: _,
			token: BlockToken::Comment(_),
		}) = value
		{
			value = self.next_token_node();
		}
		TokenNode::new(
			pos,
			BlockToken::KeyValue(
				key,
				match value {
					Some(v) => Box::new(Some(v)),
					_ => Box::new(None),
				},
			),
		)
	}

	pub fn parse_simple_string(&mut self, pos: usize, s: String) -> TokenNode {
		let mut content = s;
		let mut next_colon = false;
		loop {
			let next_token = self.peek_next_token();
			if let Some((_, token)) = next_token {
				match token {
					RawToken::SimpleString(s) => {
						content.push_str(s);
						self.pos_next();
					}
					RawToken::Sep(Sep::WhiteSpaces(s)) => {
						content.push_str(s);
						self.pos_next();
					}
					RawToken::Sep(sep) => {
						next_colon = sep == &Sep::Colon;
						self.pos_next();
						break;
					}
					_ => break,
				}
			} else {
				break;
			}
		}

		if next_colon {
			self.parse_key_value(pos, content)
		} else {
			TokenNode::new(pos, BlockToken::UnquotedString(content))
		}
	}

	pub fn parse_comment_part(&mut self, pos: usize, mark: CommentMark) -> TokenNode {
		match mark {
			CommentMark::LineBegin => {
				let comment = self.seek_and_get_string_until(vec![RawToken::Sep(Sep::NewLine)]);
				TokenNode::new(pos, BlockToken::Comment(comment))
			}
			CommentMark::BlockBegin => {
				let comment =
					self.seek_and_get_string_until(vec![RawToken::Comment(CommentMark::BlockEnd)]);
				TokenNode::new(pos, BlockToken::Comment(comment))
			}
			CommentMark::BlockEnd => self.parse_simple_string(pos, String::from("*/")),
		}
	}

	pub fn parse_sep(&mut self, _pos: usize, _sep: Sep) -> Option<TokenNode> {
		// 何か追加であったとき用メソッド、マター解析時は基本読み飛ばし
		self.next_token_node()
	}

	pub fn parse_bracket(&mut self, pos: usize, bracket: Bracket) -> Option<TokenNode> {
		if bracket.role == BracketRole::End {
			// いきなり閉じる括弧はスキップ
			return self.next_token_node();
		};

		let mut content: Vec<TokenNode> = Vec::new();
		loop {
			let next = self.peek_next_token().cloned();
			println!("next: {:#?}", next);
			match next {
				Some((pos, token)) => match token {
					RawToken::Bracket(bracket2) => {
						self.pos_next();
						if bracket2.role == BracketRole::End {
							break;
						}
						let bracket_parse = self.parse_bracket(pos, bracket2);
						match bracket_parse {
							Some(v) => content.push(v),
							_ => break,
						}
					}
					RawToken::Eos => {
						break;
					}
					_ => {
						let node = self.next_token_node();
						match node {
							Some(v) => content.push(v),
							_ => break,
						}
					}
				},
				None => {
					break;
				}
			};
		}

		Some(TokenNode::new(pos, BlockToken::Array(content)))
	}

	pub fn next_token_node(&mut self) -> Option<TokenNode> {
		let next_token = self.next_token();
		if let Some((pos, token)) = next_token {
			match token {
				RawToken::Eos => None,
				RawToken::Quote(q) => Some(self.parse_quoted_block(pos, q)),
				RawToken::SimpleString(s) => Some(self.parse_simple_string(pos, s)),
				RawToken::Comment(mark) => Some(self.parse_comment_part(pos, mark)),
				RawToken::Sep(sep) => self.parse_sep(pos, sep),
				RawToken::Bracket(bracket) => self.parse_bracket(pos, bracket),
			}
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::app::ingot::token_node::Pos;

	use super::*;

	fn tokenize_all(input: &str) -> Vec<RawTokenData> {
		let mut tokenizer = IngotTokenizer::new(String::from(input));
		let mut tokens: Vec<RawTokenData> = Vec::new();
		loop {
			let next_token = tokenizer.next_raw_token();
			match next_token.1 {
				RawToken::Eos => break,
				_ => tokens.push(next_token),
			}
		}
		tokens
	}

	#[test]
	fn test_token_nodes() {
		let tokens = tokenize_all("aaa:bbb ccc");

		let mut parser = IngotMatterTokenParser::new(tokens);
		let node = parser.next_token_node().unwrap();
		assert_eq!(
			node,
			TokenNode {
				pos: Pos { start: 0 },
				token: BlockToken::KeyValue(
					"aaa".to_string(),
					Box::new(Some(TokenNode {
						pos: Pos { start: 4 },
						token: BlockToken::UnquotedString("bbb ccc".to_string()),
					}))
				)
			}
		);
	}
}
