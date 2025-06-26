use std::io::Read;

use jiff::Timestamp;

use crate::app::{
	fs::io::read_all_from_reader,
	ingot::ingot::{RKeyList, Status, To},
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

fn split_chars(mut chars: Vec<char>, pos: usize) -> (Vec<char>, Vec<char>) {
	let after = chars.split_off(pos);
	(chars, after)
}

enum NewLineType {
	Cr,
	Lf,
	Crlf,
}

impl NewLineType {
	fn len(&self) -> usize {
		match self {
			NewLineType::Cr => 1,
			NewLineType::Lf => 1,
			NewLineType::Crlf => 2,
		}
	}
}

fn seek_next_nl(chars: &[char]) -> Option<(usize, NewLineType)> {
	let mut pos = 0;
	while let Some(c) = chars.get(pos) {
		match c {
			'\n' => return Some((pos, NewLineType::Lf)),
			'\r' => {
				if let Some('\n') = chars.get(pos + 1) {
					return Some((pos, NewLineType::Crlf));
				}
				return Some((pos, NewLineType::Cr));
			}
			_ => pos += 1,
		}
	}
	None
}

fn is_empty_chars(chars: &[char]) -> bool {
	chars.is_empty() || chars.iter().all(|c| c.is_whitespace())
}

impl IngotParser {
	pub fn split_back_matter(chars: Vec<char>) -> (Vec<char>, Vec<char>) {
		let mut pos = chars.len() - 1;
		let mut nl_count: usize = 0;
		while let Some(c) = chars.get(pos) {
			match c {
				'\n' => {
					if let Some('\r') = chars.get(pos - 1) {
						pos -= 1;
					}
					// 改行が前に2個以上ある
					if nl_count > 1 {
						return split_chars(chars, pos);
					}
					nl_count += 1;
				}
				'\r' => {
					// 改行が前に2個以上ある
					if nl_count > 1 {
						return split_chars(chars, pos);
					}
					nl_count += 1;
				}
				c if c.is_whitespace() => {}
				_ => {
					nl_count = 0;
				}
			}
			pos -= 1;
		}
		(chars, Vec::new())
	}
	pub fn set_from_key_value(result: &mut Ingot, key: String, value: Option<TokenNode>) {
		if let Some(v) = value {
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
				"status" => {
					let val = token.get_string_value_or_empty();
					result.status = val.as_str().trim().parse().unwrap_or_default();
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
				"path_url_name" | "path_name" | "url_path_name" | "post_url_name" | "page_url_name"
				| "pname" => {
					let val = token.get_string_value_or_empty();
					if !val.is_empty() {
						result.pname = val;
					}
				}
				_ => (),
			}
		}
	}

	pub fn parse<R: Read>(reader: R) -> Result<Ingot, ParseError> {
		let buffer = read_all_from_reader(reader).map_err(ParseError::IO)?;

		let mut result = Ingot::default();
		let mut tokenizer = IngotTokenizer::new(buffer.chars().collect());

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

		let (_cpos, buffer) = tokenizer.get_rest_all();

		let mut parser = IngotMatterTokenParser::new(front_matter_tokens);

		while let Some(t_node) = parser.next_token_node() {
			if let BlockToken::KeyValue(key, value) = t_node.token {
				IngotParser::set_from_key_value(&mut result, key, *value);
			}
		}

		let (mut content, back_matter) = IngotParser::split_back_matter(buffer);

		tokenizer = IngotTokenizer::new(back_matter);

		let mut back_matter_tokens: Vec<RawTokenData> = Vec::new();

		loop {
			let (pos, token) = tokenizer.next_raw_token();
			match token {
				RawToken::Eos => break,
				_ => back_matter_tokens.push((pos, token)),
			}
		}

		parser = IngotMatterTokenParser::new(back_matter_tokens);

		while let Some(t_node) = parser.next_token_node() {
			if let BlockToken::KeyValue(key, value) = t_node.token {
				IngotParser::set_from_key_value(&mut result, key, *value);
			}
		}

		// parse content
		// 最初の中身があり、後ろが空行である行がタイトル
		loop {
			match seek_next_nl(&content) {
				Some((pos, nl)) => {
					let line = &content[0..pos];
					if is_empty_chars(line) {
						// 空行はスキップ
						content.drain(0..(pos + nl.len()));
					} else {
						let cand_title_line: String = content.drain(0..pos).collect();
						content.drain(0..nl.len());
						// 次の行が空行かチェック
						match seek_next_nl(&content) {
							Some((pos, nl)) => {
								let line = &content[0..pos];
								if is_empty_chars(line) {
									// 空行ならtitleとcontentをセット
									result.title = cand_title_line;
									result.content = content.split_off(pos + nl.len()).iter().collect();
								} else {
									result.title = String::new();
									result.content = content.iter().collect::<String>();
								}
								break;
							}
							_ => {
								result.title = cand_title_line;
								result.content = content.iter().collect::<String>();
								break;
							}
						}
					}
				}
				_ => {
					result.title = content.iter().collect::<String>();
					result.content = content.iter().collect::<String>();
					break;
				}
			}
		}

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
		let mut value = self.parse_token_node(false);
		// コメントをスキップ
		if let Some(TokenNode {
			pos: _,
			token: BlockToken::Comment(_),
		}) = value
		{
			value = self.parse_token_node(false);
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

	pub fn parse_simple_string(&mut self, pos: usize, s: String, sep_colon: bool) -> TokenNode {
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
						if sep_colon {
							next_colon = sep == &Sep::Colon;
							self.pos_next();
							break;
						}
						if sep == &Sep::Colon {
							content.push(':');
							self.pos_next();
						} else {
							self.pos_next();
							break;
						}
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
			CommentMark::BlockEnd => self.parse_simple_string(pos, String::from("*/"), true),
		}
	}

	pub fn parse_sep(&mut self, _pos: usize, _sep: Sep, able_key_token: bool) -> Option<TokenNode> {
		// 何か追加であったとき用メソッド、マター解析時は基本読み飛ばし
		self.parse_token_node(able_key_token)
	}

	pub fn parse_bracket(&mut self, pos: usize, bracket: Bracket) -> Option<TokenNode> {
		if bracket.role == BracketRole::End {
			// いきなり閉じる括弧はスキップ
			return self.next_token_node();
		};

		let mut content: Vec<TokenNode> = Vec::new();
		loop {
			let next = self.peek_next_token().cloned();
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

	fn parse_token_node(&mut self, able_key_token: bool) -> Option<TokenNode> {
		if let Some((pos, token)) = self.next_token() {
			match token {
				RawToken::Eos => None,
				RawToken::Quote(q) => Some(self.parse_quoted_block(pos, q)),
				RawToken::SimpleString(s) => Some(self.parse_simple_string(pos, s, able_key_token)),
				RawToken::Comment(mark) => Some(self.parse_comment_part(pos, mark)),
				RawToken::Sep(sep) => self.parse_sep(pos, sep, able_key_token),
				RawToken::Bracket(bracket) => self.parse_bracket(pos, bracket),
			}
		} else {
			None
		}
	}

	pub fn next_token_node(&mut self) -> Option<TokenNode> {
		self.parse_token_node(true)
	}
}

#[cfg(test)]
mod tests {
	use crate::app::ingot::token_node::Pos;

	use super::*;

	fn tokenize_all(input: &str) -> Vec<RawTokenData> {
		let mut tokenizer = IngotTokenizer::new(input.chars().collect());
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
		let tokens = tokenize_all("aaa:bbb ccc, ddd:eee");

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
		let node = parser.next_token_node().unwrap();
		assert_eq!(
			node,
			TokenNode {
				pos: Pos { start: 13 },
				token: BlockToken::KeyValue(
					"ddd".to_string(),
					Box::new(Some(TokenNode {
						pos: Pos { start: 17 },
						token: BlockToken::UnquotedString("eee".to_string()),
					}))
				)
			}
		);
	}
}
