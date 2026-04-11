use serde::{Deserialize, Serialize};
use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use super::serde::{DeResult, FileType, read_deserialized_value};

#[derive(Debug, Deserialize, Serialize)]
/// Represents a tag for ingot files classification.
pub struct Tag {
	/// The unique identifier of the tag.
	pub id: usize,
	/// The name of the tag.
	pub name: String,
	/// The path name of the tag.
	pub path_name: String,
	/// The description of the tag.
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	/// Optional attributes of the tag.
	pub opt_attr: Option<BTreeMap<String, String>>,
}

impl Tag {
	/// Creates a new `Tag` instance with the given `id`, `path_name`, `name`, and `description`.
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

/// Reads the tags from the tags list file in the given directory path.
pub fn get_tags_from_dir_path(dir_path: &Path) -> Option<Vec<Tag>> {
	let file_path = tags_file_path(dir_path);
	println!("read tags from: {}", file_path.display());
	let file = std::fs::File::open(file_path).ok()?;
	read_tags(file, FileType::Ron).ok()
}

/// Returns a map of tag IDs to tags for the given list of tags.
pub fn get_index_map_from_tags(tags: &[Tag]) -> BTreeMap<usize, &Tag> {
	let mut index_tags_map: BTreeMap<usize, &Tag> = BTreeMap::new();
	for tag in tags.iter() {
		index_tags_map.insert(tag.id, tag);
	}
	index_tags_map
}
