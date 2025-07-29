use std::{
	collections::BTreeMap,
	fs,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app::{
	fs::{io::open_file_with_overwrite_mode, path::append_ext},
	serde::{FileType, write_serialized_string_all},
};

#[derive(Default, Deserialize, Serialize)]
pub struct Author {
	pub name: String,
	pub contact: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct PackInfo {
	pub name: String,
	pub authors: Vec<Author>,
	pub description: String,
	pub license: String,
	pub version: String,
}

impl PackInfo {
	pub fn new<T: Into<String>>(name: T) -> Self {
		Self {
			name: name.into(),
			authors: Vec::new(),
			description: String::new(),
			license: String::new(),
			version: String::new(),
		}
	}
}

#[derive(Deserialize, Serialize)]
pub struct PackConfig {
	pub additional_renders: Option<BTreeMap<PathBuf, PathBuf>>,
	pub static_copy: Option<BTreeMap<PathBuf, PathBuf>>,
	pub values: BTreeMap<String, String>,
}

fn default_additional_renders() -> Option<BTreeMap<PathBuf, PathBuf>> {
	let mut map = BTreeMap::new();
	map.insert(
		PathBuf::from("common.css"),
		PathBuf::from("assets/common.css"),
	);
	map.insert(
		PathBuf::from("common.js"),
		PathBuf::from("assets/common.js"),
	);
	Some(map)
}

fn default_static_copy() -> Option<BTreeMap<PathBuf, PathBuf>> {
	let mut map = BTreeMap::new();
	map.insert(PathBuf::from("assets"), PathBuf::from("assets"));
	Some(map)
}

impl Default for PackConfig {
	fn default() -> Self {
		Self {
			additional_renders: default_additional_renders(),
			static_copy: default_static_copy(),
			values: BTreeMap::new(),
		}
	}
}

pub fn norm_pack_name(set_name: String) -> String {
	set_name
		.split('/')
		.filter_map(|c| {
			let normed_c: String = c
				.trim()
				.chars()
				.filter(|c| {
					!c.is_whitespace()
						&& (c.is_alphanumeric() || ['-', '_', '@', '#', '+', '.', ':'].contains(c))
				})
				.collect();
			if normed_c.is_empty() {
				None
			} else {
				Some(normed_c)
			}
		})
		.collect()
}

pub fn create_new_pack(igata_dir_path: &Path, set_name: String) {
	let normed_set_name = norm_pack_name(set_name);
	let set_dir_path = igata_dir_path.join(&normed_set_name);

	println!("begin {normed_set_name} creation: {normed_set_name}の作成を開始します");

	match fs::create_dir(&set_dir_path) {
		Ok(()) => println!(
			"create new set directory {0}: 新しいセット用ディレクトリを作成しました {0}",
			set_dir_path.display()
		),
		Err(e) => {
			println!(
				"failed to create new set directory: セット用ディレクトリの作成に失敗しました - {e}"
			);
			return;
		}
	}

	let default_html_files = vec![
		"default", "post", "page", "list", "index", "_404", "_500", "base", "_header", "_footer",
		"main", "head", "body",
	];

	for file_name in default_html_files {
		let file_path = append_ext(set_dir_path.join(file_name), "html");
		let _ = fs::File::create(file_path);
	}

	let default_js_css = vec!["common.css", "common.js"];

	for file_name in default_js_css {
		let file_path = set_dir_path.join(file_name);
		let _ = fs::File::create(file_path);
	}

	if let Ok(writer) = open_file_with_overwrite_mode(&set_dir_path.join("pack_info.ron")) {
		let _ = write_serialized_string_all(writer, &PackInfo::new(&normed_set_name), FileType::Ron);
	}

	if let Ok(writer) = open_file_with_overwrite_mode(&set_dir_path.join("pack_config.ron")) {
		let _ = write_serialized_string_all(writer, &PackConfig::default(), FileType::Ron);
	}

	println!("finished {normed_set_name} creation: {normed_set_name}の作成を完了しました");
}
