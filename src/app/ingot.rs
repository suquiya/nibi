use std::path::PathBuf;

use jiff::Timestamp;

#[derive(Debug)]
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
	pub tag: Vec<usize>,
	pub category: Vec<usize>,
	pub to: To,
}

#[derive(Debug)]
pub enum Status {
	Draft,
	Open,
	Close,
}

#[derive(Debug)]
pub enum To {
	Post,
	Page,
	Article,
	Top,
	AsIs,
	Custom(String),
}

#[derive(Debug)]
pub enum CommentStatus {
	Open,
	Close,
}
