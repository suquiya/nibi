use std::collections::BTreeMap;

use super::{ingot::RKeyList, token_node::TokenNode};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tokens represent the separators and whitespace in the input buffer.
pub enum Sep {
	/// A comma separator.
	Comma,
	/// A colon separator.
	Colon,
	/// Whitespace characters.
	WhiteSpaces(String),
	/// A newline character.
	NewLine,
}

impl Sep {
	/// Returns the string representation of the separator.
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
/// Tokens represent the brackets' roles in the input buffer.
pub enum BracketRole {
	/// A start bracket.
	Start,
	/// An end bracket.
	End,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Tokens represent the brackets' types in the input buffer.
pub enum BracketType {
	/// A curly bracket. ()
	Curly,
	/// A square bracket. []
	Square,
	/// An angle bracket. <>
	Angle,
	/// A normal bracket. {}
	Normal,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// A bracket token represents a bracket in the input buffer.
pub struct Bracket {
	/// The role of the bracket (start or end).
	pub role: BracketRole,
	/// The type of the bracket (curly, square, angle, or normal).
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
	/// Creates a new bracket token with the given role and type.
	pub fn new(role: BracketRole, bracket: BracketType) -> Self {
		Self {
			role,
			bracket_type: bracket,
		}
	}
	/// Returns the character representation of the bracket.
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
	/// Returns the role of the bracket (start or end).
	pub fn get_role(&self) -> &BracketRole {
		&self.role
	}
	/// Returns the type of the bracket (curly, square, angle, or normal).
	pub fn get_type(&self) -> &BracketType {
		&self.bracket_type
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a quote character (single or double).
pub enum Quote {
	/// Represents a single quote character.
	Single,
	/// Represents a double quote character.
	Double,
}

impl Quote {
	/// Returns the quote character as a `char`.
	pub fn get_as_char(&self) -> char {
		match self {
			Quote::Single => '\'',
			Quote::Double => '"',
		}
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a comment mark (line begin, block begin, or block end).
pub enum CommentMark {
	/// Represents a line begin comment mark (`//`).
	LineBegin,
	/// Represents a block begin comment mark (`/*`).
	BlockBegin,
	/// Represents a block end comment mark (`*/`).
	BlockEnd,
}

impl CommentMark {
	/// Returns the comment mark as a `String`.
	pub fn get_as_string(&self) -> String {
		match self {
			CommentMark::LineBegin => "//".to_string(),
			CommentMark::BlockBegin => "/*".to_string(),
			CommentMark::BlockEnd => "*/".to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a raw token.
pub enum RawToken {
	/// Represents a quote character (single or double).
	Quote(Quote),
	/// Represents a simple string token.
	SimpleString(String),
	/// Represents a separator token (comma, colon, whitespace, or newline).
	Sep(Sep),
	/// Represents a bracket token (curly, square, angle, or normal).
	Bracket(Bracket),
	/// Represents a comment token.
	Comment(CommentMark),
	/// Represents the end of the input stream.
	Eos,
}

impl RawToken {
	/// Returns `String` that token represents.
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
/// Tuple of a raw token with its position in the input stream.
pub type RawTokenData = (usize, RawToken);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Block token
pub enum BlockToken {
	/// Token for quoted strings.
	QuotedString(Quote, String),
	/// Token for unquoted strings.
	UnquotedString(String),
	/// Token for `TokenNode` array.
	Array(Vec<TokenNode>),
	/// Token for maps.
	Map(BTreeMap<String, TokenNode>),
	/// Token for comments.
	Comment(String),
	/// Token for key-value pairs.
	KeyValue(String, Box<Option<TokenNode>>),
}

impl BlockToken {
	/// Returns the string value of the token, if it has one.
	pub fn get_string_value(&self) -> Option<&str> {
		match self {
			BlockToken::QuotedString(_, s) => Some(s),
			BlockToken::UnquotedString(s) => Some(s),
			_ => None,
		}
	}
	/// Returns the string value of the token, or an empty string if it has none.
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
