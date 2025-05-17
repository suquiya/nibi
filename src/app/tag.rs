use serde::{Deserialize, Serialize};
use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use super::serde::{DeResult, FileType, read_deserialized_value};

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

fn tags_file_path(dir_path: &Path) -> PathBuf {
	dir_path.join("tags.ron")
}

fn read_tags<R: std::io::Read>(reader: R, file_type: FileType) -> DeResult<Vec<Tag>> {
	read_deserialized_value(reader, file_type)
}

pub fn get_tags_from_dir_path(dir_path: &Path) -> Option<Vec<Tag>> {
	let file_path = tags_file_path(dir_path);
	let file = std::fs::File::open(file_path).ok()?;
	read_tags(file, FileType::Ron).ok()
}
