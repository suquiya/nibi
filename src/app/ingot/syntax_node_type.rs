#[derive(Debug, Clone, PartialEq, Eq)]
/// Syntax node type
pub enum SyntaxNodeType {
	/// Token type for quoted strings.
	QuotedString,
	/// Token type for unquoted strings.
	UnquotedString,
	/// Token type for `TokenNode` array.
	Array,
	/// Token for maps.
	Map,
	/// Token for comments.
	Comment,
	/// Token type for key-value pairs.
	KeyValuePair,
	/// Token type for Key of Pairs
	KeyValue,
	/// Token type for Value of Pairs
	Value,
}
