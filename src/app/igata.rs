use std::{
	collections::BTreeMap,
	fs,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app::fs::path::append_ext;

#[derive(Default, Deserialize, Serialize)]
pub struct Author {
	pub name: String,
	pub contact: BTreeMap<String, String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct PackConfig {
	pub name: String,
	pub authors: Vec<Author>,
	pub description: String,
	pub license: String,
	pub version: String,
	pub additional_renders: Option<BTreeMap<PathBuf, PathBuf>>,
	pub static_copy: Option<BTreeMap<PathBuf, PathBuf>>,
}

pub fn norm_set_name(set_name: String) -> String {
	set_name
		.split('/')
		.filter_map(|c| {
			let normed_c: String = c
				.trim()
				.chars()
				.filter(|c| {
					if c.is_whitespace() {
						false
					} else {
						c.is_alphanumeric() || ['-', '_', '@', '#', '+', '.', ':'].contains(c)
					}
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

pub fn create_new_set_base(igata_dir_path: &Path, set_name: String) {
	let normed_set_name = norm_set_name(set_name);
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

	println!("finished {normed_set_name} creation: {normed_set_name}の作成を完了しました");
}
