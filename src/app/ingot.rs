use std::{
	collections::VecDeque,
	io::{self, Read},
	path::PathBuf,
};

use jiff::Timestamp;
use quick_xml::reader;

#[derive(Debug, Default)]
pub struct Ingot {
	pub id: usize,
	pub author: usize,
	pub pname: String,
	pub path: PathBuf,
	pub created: Timestamp,
	pub content: String,
	pub title: String,
	pub excerpt: String,
	pub status: Status,
	pub comment_status: CommentStatus,
	pub modified: Timestamp,
	pub parent: usize,
	pub tag: RKeyList,
	pub category: RKeyList,
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

impl Default for RKeyList {
	fn default() -> Self {
		Self::Raw(Vec::default())
	}
}

#[derive(Debug, Default)]
pub enum Status {
	#[default]
	Draft,
	Open,
	Close,
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

#[derive(Debug, Default)]
pub enum CommentStatus {
	Open,
	#[default]
	Close,
}

impl Ingot {
	pub fn new(id: usize) -> Ingot {
		Ingot {
			id,
			author: usize::default(),
			pname: String::default(),
			path: PathBuf::default(),
			created: Timestamp::default(),
			content: String::default(),
			title: String::default(),
			excerpt: String::default(),
			status: Status::default(),
			comment_status: CommentStatus::default(),
			modified: Timestamp::default(),
			parent: usize::default(),
			tag: RKeyList::default(),
			category: RKeyList::default(),
			to: To::default(),
		}
	}
	pub fn parse<R: std::io::Read>(reader: R) -> Result<Ingot, ParseError> {
		let mut parser = IngotParser::new();
		parser.parse(reader)
	}
}

pub enum ParseError {
	Invalid,
	Empty,
	IO(std::io::Error),
}

struct IngotParser {}

const FRONT_MATTER_SEPARATORS: [&str; 4] = ["---", "+++", ":::", "==="];

trait Lines {
	fn from_reader<R: Read>(reader: R) -> Result<Self, io::Error>
	where
		Self: std::marker::Sized;
	fn line_pop(&mut self) -> Option<String>;
	fn trim_line_pop(&mut self) -> Option<String>;
}

impl Lines for VecDeque<String> {
	fn from_reader<R: Read>(mut reader: R) -> Result<Self, io::Error> {
		let mut buffer = String::new();
		reader.read_to_string(&mut buffer)?;
		Ok(buffer.lines().map(|s| s.to_string()).collect())
	}
	fn line_pop(&mut self) -> Option<String> {
		self.pop_front()
	}
	fn trim_line_pop(&mut self) -> Option<String> {
		self.pop_front().map(|s| s.trim().to_string())
	}
}

impl IngotParser {
	pub fn new() -> Self {
		Self {}
	}

	fn starts_with_matter_separator(line: &str) -> Option<String> {
		let first_three = &line[0..3];
		for separator in FRONT_MATTER_SEPARATORS {
			if first_three == separator {
				return Some(separator.to_string());
			}
		}
		None
	}

	pub fn parse<R: Read>(&mut self, mut reader: R) -> Result<Ingot, ParseError> {
		let mut result = Ingot::default();

		let mut lines = VecDeque::from_reader(reader).map_err(ParseError::IO)?;

		// 最初のライン起動処理
		let mut line = lines.trim_line_pop().ok_or(ParseError::Empty)?; // 何もなかったらさすがにエラー

		// 空白をremoveしても文字が出てくるまで行を読みだす、最後までずっと空ならempty判定
		while line.is_empty() {
			line = lines.trim_line_pop().ok_or(ParseError::Empty)?
		}
		// 空ではないlineの分析
		let front_matter_sep = Self::starts_with_matter_separator(&line).unwrap_or_default();
	}
}
