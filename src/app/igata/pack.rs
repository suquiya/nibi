use std::{
	collections::{BTreeMap, HashSet},
	fs,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app::{
	fs::{
		io::{open_file_with_overwrite_mode, open_file_with_read_mode},
		path::append_ext,
	},
	serde::{FileType, read_deserialized_value, write_serialized_string_all},
};

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Author {
	pub name: String,
	pub contact: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone)]
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

impl Default for PackInfo {
	fn default() -> Self {
		Self::new(String::new())
	}
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PackConfig {
	pub additional_renders: Option<BTreeMap<PathBuf, PathBuf>>,
	pub static_copy: Option<BTreeMap<PathBuf, PathBuf>>,
	pub values: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PackProperties {
	pub directory: PathBuf,
	pub info: PackInfo,
	pub config: PackConfig,
}

impl PackConfig {
	pub fn new() -> Self {
		Self::default()
	}
}

impl PackProperties {
	pub fn new(info: PackInfo, config: PackConfig, directory: PathBuf) -> Self {
		Self {
			info,
			config,
			directory,
		}
	}

	pub fn get_directory(&self) -> &PathBuf {
		&self.directory
	}

	pub fn get_info(&self) -> &PackInfo {
		&self.info
	}

	pub fn get_config(&self) -> &PackConfig {
		&self.config
	}

	pub fn get_pack_name(&self) -> &str {
		&self.info.name
	}
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

pub fn create_new_pack(igata_dir_path: &Path, pack_name: String) {
	let normed_pack_name = norm_pack_name(pack_name);
	let set_dir_path = igata_dir_path.join(&normed_pack_name);

	println!("begin {normed_pack_name} creation: {normed_pack_name}の作成を開始します");

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
		let _ = write_serialized_string_all(writer, &PackInfo::new(&normed_pack_name), FileType::Ron);
	}

	if let Ok(writer) = open_file_with_overwrite_mode(&set_dir_path.join("pack_config.ron")) {
		let _ = write_serialized_string_all(writer, &PackConfig::default(), FileType::Ron);
	}

	println!("finished {normed_pack_name} creation: {normed_pack_name}の作成を完了しました");
}

pub fn read_pack_info(pack_dir: &Path) -> Option<PackInfo> {
	let info_path = pack_dir.join("pack_info.ron");
	match open_file_with_read_mode(&info_path) {
		Ok(file) => read_deserialized_value(file, FileType::Ron).ok(),
		Err(_) => None,
	}
}

pub fn read_pack_config(pack_dir: &Path) -> Option<PackConfig> {
	let config_path = pack_dir.join("pack_config.ron");
	match open_file_with_read_mode(&config_path) {
		Ok(file) => read_deserialized_value(file, FileType::Ron).ok(),
		Err(_) => None,
	}
}

pub fn read_pack_settings(pack_dir: &Path) -> Option<PackProperties> {
	let info = read_pack_info(pack_dir)?;
	let config = read_pack_config(pack_dir)?;
	Some(PackProperties::new(info, config, pack_dir.to_path_buf()))
}

/// 指定されたpack名リストにあるpackを読み込む
/// pack名の重複チェックは行わない、重複削除してある場合がおそらく一番効率がいい
pub fn get_packs_from_names(
	pack_names: &[String],
	igata_packs_dir: &Path,
) -> BTreeMap<String, PackProperties> {
	// 探す途中で読みだしたpackのデータをキャッシュ
	let mut pack_name_cache: BTreeMap<String, PackProperties> = BTreeMap::new();
	let mut readed_paths: HashSet<PathBuf> = HashSet::new();
	// レシピで指定されたpackのデータを格納
	let mut packs = BTreeMap::<String, PackProperties>::new();

	// 鋳型パックのディレクトリ
	for pack_name in pack_names {
		if let Some(cached) = pack_name_cache.get(pack_name) {
			packs.insert(pack_name.clone(), cached.clone());
			continue;
		}
		// pack名が付いたフォルダを確認する
		let pack_name_dir = igata_packs_dir.join(pack_name);
		if let Some(pack_properties) = read_pack_settings(&pack_name_dir) {
			let r_pack_name = pack_properties.get_pack_name();
			pack_name_cache.insert(pack_name.clone(), pack_properties.clone());
			readed_paths.insert(pack_name_dir);
			if pack_name == r_pack_name {
				packs.insert(pack_name.clone(), pack_properties);
			}
		} else {
			// 見つからない／エラーが起きた場合、他のディレクトリを漁る
		}
	}

	packs
}
