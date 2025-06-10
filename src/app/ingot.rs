use std::{collections::BTreeMap, io::Read, path::PathBuf, str::FromStr};

use hcl::Block;
use jiff::Timestamp;

use super::fs::io::read_all_from_reader;

#[derive(Debug, Default)]
pub struct Ingot {
	pub id: usize,
	pub author: usize,
	pub pname: String,
	pub path: PathBuf,
	pub published: Timestamp,
	pub content: String,
	pub title: String,
	pub excerpt: String,
	pub status: Status,
	pub comment_status: CommentStatus,
	pub modified: Timestamp,
	pub tags: RKeyList,
	pub categories: RKeyList,
	pub to: To,
}

#[derive(Debug)]
pub enum RKeyRaw {
	String(String),
	Usize(usize),
}

impl Default for RKeyRaw {
	fn default() -> Self {
		Self::String(String::default())
	}
}

#[derive(Debug)]
pub enum RKeyList {
	Raw(Vec<RKeyRaw>),
	CollatedId(Vec<usize>),
}

impl Default for RKeyList {
	fn default() -> Self {
		Self::Raw(Vec::default())
	}
}

#[derive(Debug, Default)]
pub enum Status {
	#[default]
	Draft,
	Publish,
	Private,
}

impl FromStr for Status {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s.to_ascii_lowercase().as_str() {
			"draft" => Ok(Status::Draft),
			"publish" => Ok(Status::Publish),
			"private" => Ok(Status::Private),
			_ => Err(ParseError::Invalid),
		}
	}
}

#[derive(Debug, Default)]
pub enum To {
	#[default]
	Post,
	Page,
	Article,
	Top,
	AsIs,
	Custom(String),
}

impl FromStr for To {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s.to_ascii_lowercase().as_str() {
			"post" => Ok(To::Post),
			"page" => Ok(To::Page),
			"article" => Ok(To::Article),
			"top" => Ok(To::Top),
			"asis" => Ok(To::AsIs),
			_ => Ok(To::Custom(s.to_ascii_lowercase().to_string())),
		}
	}
}

#[derive(Debug, Default)]
pub enum CommentStatus {
	Open,
	#[default]
	Close,
}

impl FromStr for CommentStatus {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s {
			"open" => Ok(CommentStatus::Open),
			_ => Ok(CommentStatus::Close),
		}
	}
}

impl Ingot {
	pub fn new(id: usize) -> Ingot {
		Ingot {
			id,
			author: usize::default(),
			pname: String::default(),
			path: PathBuf::default(),
			published: Timestamp::default(),
			content: String::default(),
			title: String::default(),
			excerpt: String::default(),
			status: Status::default(),
			comment_status: CommentStatus::default(),
			modified: Timestamp::default(),
			tags: RKeyList::default(),
			categories: RKeyList::default(),
			to: To::default(),
		}
	}
	pub fn parse<R: std::io::Read>(reader: R) -> Result<Ingot, ParseError> {
		IngotParser::parse(reader)
	}
}

#[derive(Debug)]
pub enum ParseError {
	Invalid,
	Empty,
	IO(std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sep {
	Comma,
	Colon,
	WhiteSpaces(String),
	NewLine,
}

impl Sep {
	pub fn get_as_string(&self) -> String {
		match self {
			Sep::Comma => ",".to_string(),
			Sep::Colon => ":".to_string(),
			Sep::WhiteSpaces(s) => s.clone(),
			Sep::NewLine => "\n".to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketRole {
	Start,
	End,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketType {
	Curly,
	Square,
	Angle,
	Normal,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bracket {
	pub role: BracketRole,
	pub bracket_type: BracketType,
}

impl From<(BracketRole, BracketType)> for Bracket {
	fn from(value: (BracketRole, BracketType)) -> Self {
		Bracket {
			role: value.0,
			bracket_type: value.1,
		}
	}
}

impl Bracket {
	pub fn new(role: BracketRole, bracket: BracketType) -> Self {
		Self {
			role,
			bracket_type: bracket,
		}
	}

	pub fn get_as_char(&self) -> char {
		match (&self.role, &self.bracket_type) {
			(BracketRole::Start, BracketType::Curly) => '{',
			(BracketRole::End, BracketType::Curly) => '}',
			(BracketRole::Start, BracketType::Square) => '[',
			(BracketRole::End, BracketType::Square) => ']',
			(BracketRole::Start, BracketType::Angle) => '<',
			(BracketRole::End, BracketType::Angle) => '>',
			(BracketRole::Start, BracketType::Normal) => '(',
			(BracketRole::End, BracketType::Normal) => ')',
		}
	}

	pub fn get_role(&self) -> &BracketRole {
		&self.role
	}

	pub fn get_type(&self) -> &BracketType {
		&self.bracket_type
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quote {
	Single,
	Double,
}

impl Quote {
	pub fn get_as_char(&self) -> char {
		match self {
			Quote::Single => '\'',
			Quote::Double => '"',
		}
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentMark {
	LineBegin,
	BlockBegin,
	BlockEnd,
}

impl CommentMark {
	pub fn get_as_string(&self) -> String {
		match self {
			CommentMark::LineBegin => "//".to_string(),
			CommentMark::BlockBegin => "/*".to_string(),
			CommentMark::BlockEnd => "*/".to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawToken {
	Quote(Quote),
	SimpleString(String),
	Sep(Sep),
	Bracket(Bracket),
	Comment(CommentMark),
	Eos,
}

impl RawToken {
	pub fn get_as_string(&self) -> String {
		match self {
			RawToken::SimpleString(s) => s.clone(),
			RawToken::Quote(q) => q.get_as_char().to_string(),
			RawToken::Sep(s) => s.get_as_string(),
			RawToken::Bracket(bracket) => bracket.get_as_char().to_string(),
			RawToken::Comment(mark) => mark.get_as_string(),
			RawToken::Eos => "".to_string(),
		}
	}
}

type RawTokenData = (usize, RawToken);

#[derive(Debug)]
pub enum BlockToken {
	QuotedString(Quote, String),
	UnquotedString(String),
	Array(Vec<TokenNode>),
	Map(BTreeMap<String, TokenNode>),
	Comment(String),
	KeyValue(String, Box<Option<TokenNode>>),
}

// 行番号などもう少し複雑な処理が必要になる場合に備えてstructにしておく
#[derive(Debug)]
pub struct Pos {
	start: usize,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct IngotTokenizer {
	chars: Vec<char>,
	pub pos: usize,
}

const SYMBOL_CHARS: &str = "{}[]()<>,;: \t\n\r\"'/";

impl IngotTokenizer {
	/// constructor
	pub fn new(string: String) -> IngotTokenizer {
		let chars = string.chars().collect();
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

	pub fn get_rest_all(&mut self) -> String {
		if self.chars.len() > self.pos {
			self.chars[self.pos..].iter().collect()
		} else {
			String::new()
		}
	}
}

#[derive(Debug)]
pub struct IngotParser {}

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
		let value = self.next_token_node();
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
				"tags" => {}
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_raw_tokenize_basic() {
		let mut tokenizer = IngotTokenizer::new("aaa:bbb".to_string());
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
		let mut tokenizer = IngotTokenizer::new("aaa: {bbb: ccc}".to_string());
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
