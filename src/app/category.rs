use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
	pub id: usize,
	pub path_name: String,
	pub name: String,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub children: Option<Vec<Category>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ex: Option<BTreeMap<String, String>>,
}
