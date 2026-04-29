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
	/// Returns the separator as a `str`.
	pub fn as_str(&self) -> &str {
		match self {
			Sep::Comma => ",",
			Sep::Colon => ":",
			Sep::WhiteSpaces(s) => s.as_str(),
			Sep::NewLine => "\n",
		}
	}

	/// Returns the string representation of the separator.
	pub fn as_string(&self) -> String {
		match self {
			Sep::Comma => ",".to_string(),
			Sep::Colon => ":".to_string(),
			Sep::WhiteSpaces(s) => s.clone(),
			Sep::NewLine => "\n".to_string(),
		}
	}

	/// Add the string representation of the separator to the given string.
	pub fn add_to_string(&self, s: &mut String) {
		match self {
			Sep::Comma => s.push(','),
			Sep::Colon => s.push(':'),
			Sep::WhiteSpaces(sep) => s.push_str(sep),
			Sep::NewLine => s.push('\n'),
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
	pub fn as_char(&self) -> char {
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

	/// Add the string representation of the bracket to the given string.
	pub fn add_to_string(&self, s: &mut String) {
		s.push(self.as_char());
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
	pub fn as_char(&self) -> char {
		match self {
			Quote::Single => '\'',
			Quote::Double => '"',
		}
	}

	/// Add the string representation of the quote to the given string.
	pub fn add_to_string(&self, s: &mut String) {
		s.push(self.as_char());
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
	/// Returns the comment mark as a `str`.
	pub fn as_str(&self) -> &str {
		match self {
			CommentMark::LineBegin => "//",
			CommentMark::BlockBegin => "/*",
			CommentMark::BlockEnd => "*/",
		}
	}

	/// Returns the comment mark as a `String`.
	pub fn as_string(&self) -> String {
		self.as_str().to_string()
	}

	/// Add the string representation of the comment mark to the given string.
	pub fn add_to_string(&self, s: &mut String) {
		s.push_str(self.as_str());
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
			RawToken::Quote(q) => q.as_char().to_string(),
			RawToken::Sep(s) => s.as_string(),
			RawToken::Bracket(bracket) => bracket.as_char().to_string(),
			RawToken::Comment(mark) => mark.as_string(),
			RawToken::Eos => "".to_string(),
		}
	}

	/// Appends the string representation of this token to the given string.
	pub fn add_to_string(&self, dest: &mut String) {
		match self {
			RawToken::SimpleString(s) => dest.push_str(s),
			RawToken::Quote(q) => q.add_to_string(dest),
			RawToken::Sep(s) => s.add_to_string(dest),
			RawToken::Bracket(bracket) => bracket.add_to_string(dest),
			RawToken::Comment(mark) => mark.add_to_string(dest),
			RawToken::Eos => {}
		}
	}
}
/// Tuple of a raw token with its position in the input stream.
pub type RawTokenData = (usize, RawToken);
