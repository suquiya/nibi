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
/// Represents an author of a pack.
pub struct Author {
	/// The name of the author.
	pub name: String,
	/// The contact information of the author.
	pub contact: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone)]
/// Represents the information of a pack.
pub struct PackInfo {
	/// The name of the pack.
	pub name: String,
	/// The authors of the pack.
	pub authors: Vec<Author>,
	/// The description of the pack.
	pub description: String,
	/// The license of the pack.
	pub license: String,
	/// The version of the pack.
	pub version: String,
}

impl PackInfo {
	/// Creates a new `PackInfo` with the given name.
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
/// Represents the configuration of a pack.
pub struct PackConfig {
	/// Additional renders to perform on the pack.
	pub additional_renders: Option<BTreeMap<PathBuf, PathBuf>>,
	/// Static files to copy into the pack.
	pub static_copy: Option<BTreeMap<PathBuf, PathBuf>>,
	/// Values to replace in the pack.
	pub values: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone)]
/// Represents the properties of a pack.
pub struct PackProperties {
	/// The directory of the pack.
	pub directory: PathBuf,
	/// The info of the pack.
	pub info: PackInfo,
	/// The config of the pack.
	pub config: PackConfig,
}

impl PackConfig {
	/// Creates a new `PackConfig` with default values.
	pub fn new() -> Self {
		Self::default()
	}
}

impl PackProperties {
	/// Creates a new `PackProperties` with the given info, config, and directory.
	pub fn new(info: PackInfo, config: PackConfig, directory: PathBuf) -> Self {
		Self {
			info,
			config,
			directory,
		}
	}
	/// Returns a reference to the directory of the pack.
	pub fn get_directory(&self) -> &Path {
		&self.directory
	}
	/// Returns a reference to the info of the pack.
	pub fn get_info(&self) -> &PackInfo {
		&self.info
	}
	/// Returns a reference to the config of the pack.
	pub fn get_config(&self) -> &PackConfig {
		&self.config
	}
	/// Returns the name of the pack.
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
/// Normalizes the given pack name by removing invalid characters and trimming whitespace.
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
/// Creates a new pack with the given name in the given Igata directory path.
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
/// Reads the pack info from the given pack directory path, if the pack info file exists.
pub fn read_pack_info(pack_dir: &Path) -> Option<PackInfo> {
	let info_path = pack_dir.join("pack_info.ron");
	match open_file_with_read_mode(&info_path) {
		Ok(file) => read_deserialized_value(file, FileType::Ron).ok(),
		Err(_) => None,
	}
}

/// Reads the pack config from the given pack directory path, if the pack config file exists.
pub fn read_pack_config(pack_dir: &Path) -> Option<PackConfig> {
	let config_path = pack_dir.join("pack_config.ron");
	match open_file_with_read_mode(&config_path) {
		Ok(file) => read_deserialized_value(file, FileType::Ron).ok(),
		Err(_) => None,
	}
}
/// Reads the pack settings (info and config) from the given pack directory path, if both files exist.
pub fn read_pack_settings(pack_dir: &Path) -> Option<PackProperties> {
	let info = read_pack_info(pack_dir)?;
	let config = read_pack_config(pack_dir)?;
	Some(PackProperties::new(info, config, pack_dir.to_path_buf()))
}

/// Gets the packs from the given pack names and Igata directory path.
/// No duplicate checking is performed.
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
