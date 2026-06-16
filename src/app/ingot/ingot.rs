use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
	str::FromStr,
};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::app::{
	category::Category,
	fs::{
		io::open_file_with_read_mode,
		path::{is_file_ends_with, iter_all_paths},
	},
	tag::Tag,
};

use super::{error::ParseError, parser::IngotParser};

#[derive(Debug, Default)]
/// Ingot struct. Represents an site page contents data.
pub struct Ingot {
	/// The ingot ID.
	pub id: usize,
	/// The author ID.
	pub author: usize,
	/// The page name.
	pub pname: String,
	/// The file path.
	pub path: PathBuf,
	/// The published timestamp.
	pub published: Timestamp,
	/// The content.
	pub content: String,
	/// The title.
	pub title: String,
	/// The excerpt.
	pub excerpt: String,
	/// The ingot's status (e.g. draft, publish, private).
	pub status: Status,
	/// The comment status (e.g. open, close).
	pub comment_status: CommentStatus,
	/// The updated timestamp.
	pub updated: Timestamp,
	/// The tags.
	pub tags: RKeyList,
	/// The categories.
	pub categories: RKeyList,
	/// The ingot build type(e.g. post, page, article, top, as-is, custom).
	pub to: To,
}

#[derive(Debug)]
/// Enum for a relational and raw key value.
pub enum RKeyRaw {
	/// A string value.
	String(String),
	/// A usize value.
	Usize(usize),
}

impl Default for RKeyRaw {
	fn default() -> Self {
		Self::String(String::default())
	}
}

#[derive(Debug)]
/// Enum for a list of relational and raw key values.
pub enum RKeyList {
	/// A list of raw key values.
	Raw(Vec<RKeyRaw>),
	/// A list of collated IDs (after resolving relation).
	CollatedId(Vec<usize>),
}

impl From<String> for RKeyRaw {
	fn from(value: String) -> Self {
		let trimed = value.trim().trim_matches(['"', '\'']);
		if let Ok(id) = trimed.parse::<usize>() {
			RKeyRaw::Usize(id)
		} else {
			RKeyRaw::String(trimed.to_string())
		}
	}
}

impl Default for RKeyList {
	fn default() -> Self {
		Self::Raw(Vec::default())
	}
}

impl From<String> for RKeyList {
	fn from(value: String) -> Self {
		let raw_keys = value
			.split(',')
			.collect::<Vec<_>>()
			.iter_mut()
			.map(|s| RKeyRaw::from(s.to_owned()))
			.collect::<Vec<_>>();
		Self::Raw(raw_keys)
	}
}

impl From<Vec<String>> for RKeyList {
	fn from(value: Vec<String>) -> Self {
		let raw_keys = value.into_iter().map(RKeyRaw::from).collect::<Vec<_>>();
		Self::Raw(raw_keys)
	}
}

#[derive(Debug, Default, strum::Display)]
/// Enum for the page(ingot) status.
pub enum Status {
	#[default]
	/// Draft
	Draft,
	/// Publish
	Publish,
	/// Private
	Private,
}

impl FromStr for Status {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s.to_ascii_lowercase().as_str() {
			"draft" => Ok(Status::Draft),
			"publish" => Ok(Status::Publish),
			"private" => Ok(Status::Private),
			_ => Err(ParseError::Invalid),
		}
	}
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
/// Enum for the article(ingot) build type.
pub enum To {
	#[default]
	/// Post
	Post,
	/// Page
	Page,
	/// Article
	Article,
	/// Top page
	Top,
	/// As is
	AsIs,
	/// Custom
	Custom(String),
}

impl From<&str> for To {
	fn from(s: &str) -> To {
		match s.to_ascii_lowercase().as_str() {
			"post" => To::Post,
			"page" => To::Page,
			"article" => To::Article,
			"top" => To::Top,
			"index" => To::Top,
			"asis" => To::AsIs,
			_ => To::Custom(s.to_ascii_lowercase().to_string()),
		}
	}
}

#[derive(Debug, Default, strum::Display)]
/// Enum for the pags's comment status.
pub enum CommentStatus {
	/// Open
	Open,
	/// Close
	#[default]
	Close,
}

impl FromStr for CommentStatus {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s {
			"open" | "Open" | "OPEN" => Ok(CommentStatus::Open),
			_ => Ok(CommentStatus::Close),
		}
	}
}

impl Ingot {
	/// Creates a new `Ingot` instance with the given ID.
	pub fn new(id: usize) -> Ingot {
		Ingot {
			id,
			author: usize::default(),
			pname: String::default(),
			path: PathBuf::default(),
			published: Timestamp::default(),
			content: String::default(),
			title: String::default(),
			excerpt: String::default(),
			status: Status::default(),
			comment_status: CommentStatus::default(),
			updated: Timestamp::default(),
			tags: RKeyList::default(),
			categories: RKeyList::default(),
			to: To::default(),
		}
	}
	/// Reads and parses an `Ingot` from a reader.
	pub fn read<R: std::io::Read>(reader: R) -> Result<Ingot, ParseError> {
		IngotParser::parse(reader)
	}

	/// Collates the IDs of the categories and tags in the `Ingot`.
	pub fn collate_ids(
		&mut self,
		categories_index_map: &BTreeMap<usize, &Category>,
		tags_index_map: &BTreeMap<usize, &Tag>,
	) {
		if let RKeyList::Raw(raw) = &self.categories {
			self.categories = RKeyList::CollatedId(
				raw.iter()
					.filter_map(|r| match r {
						RKeyRaw::Usize(id) => {
							if categories_index_map.contains_key(id) {
								Some(*id)
							} else {
								None
							}
						}
						RKeyRaw::String(name) => {
							categories_index_map.iter().find_map(|(id, category)| {
								if &category.name == name || &category.path_name == name {
									Some(*id)
								} else {
									None
								}
							})
						}
					})
					.collect(),
			);
		}
		if let RKeyList::Raw(raw) = &self.tags {
			self.tags = RKeyList::CollatedId(
				raw.iter()
					.filter_map(|r| match r {
						RKeyRaw::Usize(id) => {
							if tags_index_map.contains_key(id) {
								Some(*id)
							} else {
								None
							}
						}
						RKeyRaw::String(name) => tags_index_map.iter().find_map(|(id, tag)| {
							if &tag.name == name || &tag.path_name == name {
								Some(*id)
							} else {
								None
							}
						}),
					})
					.collect(),
			)
		}
	}
}

/// Reads an ingot from a file and collates its category and tag IDs using the provided index maps.
pub fn get_ingot_from_path_and_collate_id_maps(
	path: &Path,
	categories_index_map: &BTreeMap<usize, &Category>,
	tags_index_map: &BTreeMap<usize, &Tag>,
) -> Result<Ingot, ParseError> {
	let reader = open_file_with_read_mode(path).map_err(ParseError::IO)?;
	Ingot::read(reader).map(|mut ingot| {
		ingot.collate_ids(categories_index_map, tags_index_map);
		ingot
	})
}

/// Gets `BTreeMap<usize, (PathBuf, Ingot)>` from a directory path and collates its category and tag IDs using the provided index maps.
pub fn get_ingots_from_dir_and_collate_id_maps(
	root: &Path,
	categories_index_map: &BTreeMap<usize, &Category>,
	tags_index_map: &BTreeMap<usize, &Tag>,
) -> BTreeMap<usize, (PathBuf, Ingot)> {
	let mut ingots: BTreeMap<usize, (PathBuf, Ingot)> = BTreeMap::new();
	let ext = ".ingot";
	let mut duplicate: Vec<(PathBuf, Ingot)> = Vec::new();
	for entry in iter_all_paths(root).filter(|e| is_file_ends_with(e, ext)) {
		let reader = open_file_with_read_mode(entry.path()).unwrap();
		match Ingot::read(reader) {
			Ok(mut ingot) => {
				// ingotのカテゴリとタグを照合
				ingot.collate_ids(categories_index_map, tags_index_map);
				let value = ingots.insert(ingot.id, (entry.path().to_path_buf(), ingot));
				if let Some(value) = value {
					duplicate.push(value);
				}
			}
			Err(e) => {
				println!("{}: {}", entry.path().display(), e);
			}
		}
	}

	if !duplicate.is_empty() {
		println!("Ingots have duplicate id are found: {}", duplicate.len());
		let mut max_id: usize = *ingots.last_key_value().unwrap().0;
		for (path, mut ingot) in duplicate.into_iter() {
			max_id += 1;
			ingot.id = max_id;
			ingots.insert(max_id, (path, ingot));
		}
	}
	ingots
}
