use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
	pub id: usize,
	pub name: String,
	pub path_name: String,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ex: Option<BTreeMap<String, String>>,
}
