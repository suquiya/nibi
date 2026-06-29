#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicTokenType {
	/// Literal
	Literal,

	// Brackets
	/// Bracket's Begin
	BracketBegin,
	/// Bracket's End
	BracketEnd,

	// Separators
	/// Comma
	Comma,
	/// Colon
	Colon,
	/// Whitespace
	Whitespaces,
	/// NewLine
	NewLine,

	// Quote
	/// Single Quote
	SingleQuote,
	/// Double Quote
	DoubleQuote,

	// CommentMarkers
	/// Single Line Comment
	LineCommentBegin,
	/// Multi Line Comment
	BlockCommentBegin,
	/// Multi Line Comment End
	BlockCommentEnd,

	/// Eos
	Eos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicToken {
	token_type: BasicTokenType,
	content: String,
}

impl BasicToken {
	fn new(token_type: BasicTokenType, content: String) -> Self {
		BasicToken {
			token_type,
			content,
		}
	}

	pub fn literal(content: String) -> Self {
		Self::new(BasicTokenType::Literal, content)
	}

	pub fn bracket_begin(content: String) -> Self {
		Self::new(BasicTokenType::BracketBegin, content)
	}

	pub fn bracket_end(content: String) -> Self {
		Self::new(BasicTokenType::BracketEnd, content)
	}

	pub fn comma() -> Self {
		Self::new(BasicTokenType::Comma, ",".to_string())
	}

	pub fn colon() -> Self {
		Self::new(BasicTokenType::Colon, ":".to_string())
	}

	pub fn semicolon() -> Self {
		Self::new(BasicTokenType::Colon, ";".to_string())
	}

	pub fn whitespaces(content: String) -> Self {
		Self::new(BasicTokenType::Whitespaces, content)
	}

	pub fn newline(content: String) -> Self {
		Self::new(BasicTokenType::NewLine, content)
	}

	pub fn single_quote() -> Self {
		Self::new(BasicTokenType::SingleQuote, "\'".to_string())
	}

	pub fn double_quote() -> Self {
		Self::new(BasicTokenType::DoubleQuote, "\"".to_string())
	}

	pub fn line_comment_begin() -> Self {
		Self::new(BasicTokenType::LineCommentBegin, "//".to_string())
	}

	pub fn block_comment_begin() -> Self {
		Self::new(BasicTokenType::BlockCommentBegin, "/*".to_string())
	}

	pub fn block_comment_end() -> Self {
		Self::new(BasicTokenType::BlockCommentEnd, "*/".to_string())
	}

	pub fn eos() -> Self {
		Self::new(BasicTokenType::Eos, "".to_string())
	}

	pub fn token_type(&self) -> &BasicTokenType {
		&self.token_type
	}

	pub fn content(&self) -> &str {
		&self.content
	}

	pub fn content_char(&self) -> Option<char> {
		self.content.chars().next()
	}
}
