use std::{path::PathBuf, str::FromStr};

use jiff::Timestamp;

use super::{error::ParseError, parser::IngotParser};

#[derive(Debug, Default)]
pub struct Ingot {
	pub id: usize,
	pub author: usize,
	pub pname: String,
	pub path: PathBuf,
	pub published: Timestamp,
	pub content: String,
	pub title: String,
	pub excerpt: String,
	pub status: Status,
	pub comment_status: CommentStatus,
	pub updated: Timestamp,
	pub tags: RKeyList,
	pub categories: RKeyList,
	pub to: To,
}

#[derive(Debug)]
pub enum RKeyRaw {
	String(String),
	Usize(usize),
}

impl Default for RKeyRaw {
	fn default() -> Self {
		Self::String(String::default())
	}
}

#[derive(Debug)]
pub enum RKeyList {
	Raw(Vec<RKeyRaw>),
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

#[derive(Debug, Default)]
pub enum Status {
	#[default]
	Draft,
	Publish,
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

#[derive(Debug, Default)]
pub enum To {
	#[default]
	Post,
	Page,
	Article,
	Top,
	AsIs,
	Custom(String),
}

impl From<&str> for To {
	fn from(s: &str) -> To {
		match s.to_ascii_lowercase().as_str() {
			"post" => To::Post,
			"page" => To::Page,
			"article" => To::Article,
			"top" => To::Top,
			"asis" => To::AsIs,
			_ => To::Custom(s.to_ascii_lowercase().to_string()),
		}
	}
}

#[derive(Debug, Default)]
pub enum CommentStatus {
	Open,
	#[default]
	Close,
}

impl FromStr for CommentStatus {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s {
			"open" => Ok(CommentStatus::Open),
			_ => Ok(CommentStatus::Close),
		}
	}
}

impl Ingot {
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
	pub fn read<R: std::io::Read>(reader: R) -> Result<Ingot, ParseError> {
		IngotParser::parse(reader)
	}
}
