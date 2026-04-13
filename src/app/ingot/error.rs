#[derive(Debug)]
/// Represents an error that can occur during ingot parsing.
pub enum ParseError {
	/// The format is invalid.
	Invalid,
	/// The data is empty.
	Empty,
	/// An I/O error occurred.
	IO(std::io::Error),
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParseError::Invalid => write!(f, "ParseError: format is invalid"),
			ParseError::Empty => write!(f, "ParseError: data is empty"),
			ParseError::IO(err) => write!(f, "IO: {err}"),
		}
	}
}
