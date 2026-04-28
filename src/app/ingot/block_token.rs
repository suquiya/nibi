use std::collections::BTreeMap;

use crate::app::{
	ingot::{ingot::RKeyList, token_node::TokenNode},
	raw_token::token::Quote,
};

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
