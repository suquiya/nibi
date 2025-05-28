use std::{
	collections::{BTreeMap, VecDeque},
	io::{self, Read},
	path::PathBuf,
	str::FromStr,
};

use hcl::value;
use jiff::Timestamp;
use ron::{
	Map, Number, Value,
	value::{F32, F64},
};

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
	pub modified: Timestamp,
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

impl TryFrom<Value> for Status {
	type Error = ParseError;
	fn try_from(value: Value) -> Result<Self, ParseError> {
		match value {
			Value::String(s) => Status::from_str(&s),
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

impl FromStr for To {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, ParseError> {
		match s.to_ascii_lowercase().as_str() {
			"post" => Ok(To::Post),
			"page" => Ok(To::Page),
			"article" => Ok(To::Article),
			"top" => Ok(To::Top),
			"asis" => Ok(To::AsIs),
			_ => Ok(To::Custom(s.to_string())),
		}
	}
}

impl TryFrom<Value> for To {
	type Error = ParseError;
	fn try_from(value: Value) -> Result<Self, ParseError> {
		match value {
			Value::String(s) => To::from_str(&s),
			_ => Err(ParseError::Invalid),
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

impl TryFrom<Value> for CommentStatus {
	type Error = ParseError;
	fn try_from(value: Value) -> Result<Self, ParseError> {
		match value {
			Value::String(s) => CommentStatus::from_str(&s),
			_ => Err(ParseError::Invalid),
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
			modified: Timestamp::default(),
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

macro_rules! set_if_some {
	($res:ident,$key: ident, $expr: expr) => {{
		if let Some($key) = $expr {
			$res.$key = $key;
		};
		true
	}};
}

impl IngotParser {
	pub fn new() -> Self {
		Self {}
	}

	fn get_string_from_value(&mut self, value: Value) -> Option<String> {
		match value {
			Value::String(value) => Some(value),
			Value::Number(value) => match value {
				Number::F32(F32(value)) => Some(value.to_string()),
				Number::F64(F64(value)) => Some(value.to_string()),
				Number::U8(value) => Some(value.to_string()),
				Number::U16(value) => Some(value.to_string()),
				Number::U32(value) => Some(value.to_string()),
				Number::U64(value) => Some(value.to_string()),
				Number::I8(value) => Some(value.to_string()),
				Number::I16(value) => Some(value.to_string()),
				Number::I32(value) => Some(value.to_string()),
				Number::I64(value) => Some(value.to_string()),
			},
			_ => None,
		}
	}

	fn get_pathbuf_from_value(&mut self, value: Value) -> Option<PathBuf> {
		match self.get_string_from_value(value) {
			Some(value) => Some(PathBuf::from(value)),
			_ => None,
		}
	}

	fn conv_map_to_string_key(&mut self, map: Map) -> BTreeMap<String, Value> {
		map.into_iter()
			.filter_map(|(k, v)| match self.get_string_from_value(k) {
				Some(k) => Some((k, v)),
				_ => None,
			})
			.collect()
	}

	fn get_usize_from_value(&mut self, value: Value) -> Option<usize> {
		match value {
			Value::String(val) => match val.parse::<usize>() {
				Ok(id) => Some(id),
				Err(_) => None,
			},
			Value::Number(num) => self.get_usize_from_value_number(num),
			_ => None,
		}
	}

	fn get_usize_from_value_number(&mut self, value: Number) -> Option<usize> {
		match value {
			Number::U8(k) => Some(k.into()),
			Number::U16(k) => Some(k.into()),
			Number::U32(k) => k.try_into().ok(),
			Number::U64(k) => k.try_into().ok(),
			Number::I8(k) => k.try_into().ok(),
			Number::I16(k) => k.try_into().ok(),
			Number::I32(k) => k.try_into().ok(),
			Number::I64(k) => k.try_into().ok(),
			_ => None,
		}
	}

	fn get_timestamp_from_value(&mut self, value: Value) -> Option<Timestamp> {
		self
			.get_string_from_value(value)
			.and_then(|value| value.parse().ok())
	}

	fn get_rkey_list_from_value(&mut self, value: Value) -> Option<RKeyList> {
		match value {
			Value::Seq(list) => {
				let l: Vec<RKeyRaw> = list
					.into_iter()
					.filter_map(|v| match self.get_usize_from_value(v.clone()) {
						Some(id) => Some(RKeyRaw::Usize(id)),
						_ => self.get_string_from_value(v).map(RKeyRaw::String),
					})
					.collect();
				Some(RKeyList::Raw(l))
			}
			_ => None,
		}
	}

	fn set_from_key_value(&mut self, key: String, value: Value, result: &mut Ingot) -> bool {
		match key.as_str() {
			"id" => set_if_some!(result, id, self.get_usize_from_value(value)),
			"author" => set_if_some!(result, author, self.get_usize_from_value(value)),
			"pname" => set_if_some!(result, pname, self.get_string_from_value(value)),
			"path" => set_if_some!(result, path, self.get_pathbuf_from_value(value)),
			"published" => set_if_some!(result, published, self.get_timestamp_from_value(value)),
			"content" => set_if_some!(result, content, self.get_string_from_value(value)),
			"title" => set_if_some!(result, title, self.get_string_from_value(value)),
			"excerpt" => set_if_some!(result, excerpt, self.get_string_from_value(value)),
			"status" => set_if_some!(result, status, value.try_into().ok()),
			"comment_status" => set_if_some!(result, comment_status, value.try_into().ok()),
			"modified" => set_if_some!(result, modified, self.get_timestamp_from_value(value)),
			"tag" => set_if_some!(result, tag, self.get_rkey_list_from_value(value)),
			"category" => set_if_some!(result, category, self.get_rkey_list_from_value(value)),
			"to" => set_if_some!(result, to, value.try_into().ok()),
			_ => false,
		}
	}

	fn set_from_map(&mut self, map: Map, result: &mut Ingot) -> bool {
		let map = self.conv_map_to_string_key(map);
		let mut hit = false;
		for (k, v) in map {
			let r = self.set_from_key_value(k, v, result);
			hit = hit || r;
		}
		hit
	}

	fn set_matter(&mut self, matter_lines: &Vec<&str>, result: &mut Ingot) -> bool {
		let mut buffer = String::from("}");
		for line in matter_lines {
			let trim_end_line = line.trim_end();
			if trim_end_line.ends_with(",") {
				buffer.push_str(trim_end_line);
				buffer.push('\n');
			} else {
				buffer.push_str(trim_end_line);
				buffer.push_str(",\n");
			}
		}
		match ron::from_str(&buffer) {
			Ok(Value::Map(m)) => {
				self.set_from_map(m, result);
				true
			}
			_ => false,
		}
	}

	fn seek_to_first_not_empty_line<'a>(
		&mut self,
		lines: &mut VecDeque<&'a str>,
	) -> Option<&'a str> {
		while let Some(line) = lines.pop_front() {
			if line.is_empty() {
				continue;
			}
			return Some(line);
		}
		None
	}

	fn get_content_from_lines(&mut self, lines: VecDeque<&str>) -> String {
		lines.into_iter().collect::<Vec<&str>>().join("\n")
	}

	pub fn parse<R: Read>(&mut self, mut reader: R) -> Result<Ingot, ParseError> {
		let mut result = Ingot::default();

		let mut buffer = String::new();
		reader.read_to_string(&mut buffer).map_err(ParseError::IO)?;

		let mut lines: VecDeque<&str> = buffer.lines().collect();

		let mut front_matter_lines: Vec<&str> = Vec::new();

		while let Some(line) = lines.pop_front() {
			if line.is_empty() {
				break;
			}
			front_matter_lines.push(line);
		}

		if !self.set_matter(&front_matter_lines, &mut result) {
			let mut queue = VecDeque::from(front_matter_lines);
			queue.append(&mut lines);
			lines = queue;
		}

		let mut back_matter_lines: Vec<&str> = Vec::new();

		let mut prev_is_empty = false;
		while let Some(line) = lines.pop_back() {
			if line.is_empty() {
				if prev_is_empty {
					break;
				}
				prev_is_empty = true;
				continue;
			}
			prev_is_empty = false;
			back_matter_lines.push(line);
		}

		if !self.set_matter(&back_matter_lines, &mut result) {
			lines.append(&mut VecDeque::from(back_matter_lines));
		}

		let fline = self.seek_to_first_not_empty_line(&mut lines);

		match fline {
			None => Ok(result),
			Some(fline) => {
				if result.title.is_empty() {
					let next_line = lines.pop_front();
					match next_line {
						None => {
							result.title = fline.to_string();
						}
						Some(line) if line.trim().is_empty() => {
							result.title = fline.to_string();
						}
						Some(nl) => {
							lines.push_front(nl);
							lines.push_front(fline);
						}
					}
					result.content = self.get_content_from_lines(lines);
					Ok(result)
				} else {
					result.content = self.get_content_from_lines(lines);
					Ok(result)
				}
			}
		}
	}
}
