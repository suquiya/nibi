use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
	pub id: usize,
	pub name: String,
	pub path_name: String,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub opt_attr: Option<BTreeMap<String, String>>,
}

impl Tag {
	pub fn new(id: usize, path_name: String, name: String, description: String) -> Self {
		Self {
			id,
			path_name,
			name,
			description,
			opt_attr: None,
		}
	}
}
