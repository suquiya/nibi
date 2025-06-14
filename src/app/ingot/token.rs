use std::collections::BTreeMap;

use super::{ingot::RKeyList, token_node::TokenNode};

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

pub type RawTokenData = (usize, RawToken);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockToken {
	QuotedString(Quote, String),
	UnquotedString(String),
	Array(Vec<TokenNode>),
	Map(BTreeMap<String, TokenNode>),
	Comment(String),
	KeyValue(String, Box<Option<TokenNode>>),
}

impl BlockToken {
	pub fn get_string_value(&self) -> Option<&str> {
		match self {
			BlockToken::QuotedString(_, s) => Some(s),
			BlockToken::UnquotedString(s) => Some(s),
			_ => None,
		}
	}

	pub fn get_string_value_or_empty(&self) -> String {
		match self {
			BlockToken::QuotedString(_, s) => s.clone(),
			BlockToken::UnquotedString(s) => s.clone(),
			_ => String::from(""),
		}
	}
}

impl From<BlockToken> for RKeyList {
	fn from(value: BlockToken) -> RKeyList {
		match value {
			BlockToken::QuotedString(_, s) => RKeyList::from(s),
			BlockToken::UnquotedString(s) => RKeyList::from(s),
			BlockToken::Array(token_node_array) => {
				let raw_strings: Vec<String> = token_node_array
					.iter()
					.filter_map(|tn| tn.get_string_value().map(|s| s.to_string()))
					.collect();
				RKeyList::from(raw_strings)
			}
			_ => RKeyList::Raw(Vec::default()),
		}
	}
}
