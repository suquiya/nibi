use std::fs::{self, File};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::VariantNames;

use super::fs::io::{new_empty_file, open_file_with_overwrite_mode, open_file_with_read_mode};
use super::serde::{
	DeResult, FileType, SerResult, read_deserialized_value, write_serialized_string_all,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	project_name: String,
	site_name: String,
	#[serde(default, skip_serializing_if = "DirConf::is_default")]
	dir_conf: DirConf,
	#[serde(default, skip_serializing_if = "is_default_recipe_name")]
	recipe: String,
}

pub fn default_project_name() -> String {
	String::from("nibi_project")
}

pub fn default_site_name() -> String {
	String::from("nibi_site")
}

fn recipe_path_default() -> String {
	String::from("recipe")
}

fn is_default_recipe_name(path: &String) -> bool {
	path == &recipe_path_default()
}

impl Default for Config {
	fn default() -> Self {
		Self {
			project_name: default_project_name(),
			site_name: default_site_name(),
			dir_conf: DirConf::default(),
			recipe: recipe_path_default(),
		}
	}
}

impl Config {
	pub fn new(project_name: String, site_name: String) -> Self {
		Self {
			project_name,
			site_name,
			dir_conf: DirConf::default(),
			recipe: recipe_path_default(),
		}
	}

	pub fn project_name<T: Into<String>>(mut self, proj_name: T) -> Self {
		self.project_name = proj_name.into();
		self
	}

	pub fn site_name<T: Into<String>>(mut self, site_name: T) -> Self {
		self.site_name = site_name.into();
		self
	}

	pub fn site_name_ref(&self) -> &String {
		&self.site_name
	}

	pub fn to_file(&self, file: &File, file_type: FileType) -> SerResult<()> {
		write_serialized_string_all(file, self, file_type)
	}

	pub fn read<R: std::io::Read>(reader: R, file_type: FileType) -> DeResult<Config> {
		read_deserialized_value(reader, file_type)
	}

	pub fn get_dir_conf(&self) -> &DirConf {
		&self.dir_conf
	}
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DirConf {
	#[serde(
		default = "site_path_default",
		skip_serializing_if = "is_site_path_default"
	)]
	site: PathBuf, // 出力先
	#[serde(
		default = "zairyo_path_default",
		skip_serializing_if = "is_zairyo_path_default"
	)]
	zairyo: PathBuf, // 生地データなどの材料
	#[serde(
		default = "igata_path_default",
		skip_serializing_if = "is_igata_path_default"
	)]
	igata: PathBuf, // 鋳型
	#[serde(
		default = "gears_path_default",
		skip_serializing_if = "is_gears_path_default"
	)]
	gears: PathBuf, //アドオン設定置き予定
}

fn site_path_default() -> PathBuf {
	PathBuf::from(String::from("site"))
}

fn is_site_path_default(path: &Path) -> bool {
	path == site_path_default()
}

fn zairyo_path_default() -> PathBuf {
	PathBuf::from(String::from("zairyo"))
}

fn is_zairyo_path_default(path: &Path) -> bool {
	path == zairyo_path_default()
}

fn igata_path_default() -> PathBuf {
	PathBuf::from(String::from("igata"))
}

fn is_igata_path_default(path: &Path) -> bool {
	path == igata_path_default()
}

fn gears_path_default() -> PathBuf {
	PathBuf::from(String::from("gears"))
}

fn is_gears_path_default(path: &Path) -> bool {
	path == gears_path_default()
}

impl Default for DirConf {
	fn default() -> Self {
		Self {
			site: site_path_default(),
			zairyo: zairyo_path_default(),
			igata: igata_path_default(),
			gears: gears_path_default(),
		}
	}
}

impl DirConf {
	pub fn create_src_dirs(&self, parent_path: &Path) -> Result<(), Vec<(IOError, &PathBuf)>> {
		let mut errs = vec![];
		for path in [&self.zairyo, &self.igata, &self.gears] {
			if let Err(e) = fs::create_dir(parent_path.join(path)) {
				errs.push((e, path));
			}
		}
		if errs.is_empty() { Ok(()) } else { Err(errs) }
	}

	pub fn is_default(&self) -> bool {
		let default = DirConf::default();
		self == &default
	}

	pub fn get_zairyo_path(&self, parent_path: &Path) -> PathBuf {
		parent_path.join(&self.zairyo)
	}

	pub fn get_igata_path(&self, parent_path: &Path) -> PathBuf {
		parent_path.join(&self.igata)
	}

	pub fn get_gears_path(&self, parent_path: &Path) -> PathBuf {
		parent_path.join(&self.gears)
	}

	pub fn get_site_path(&self, parent_path: &Path) -> PathBuf {
		parent_path.join(&self.site)
	}
}

pub fn default_config_file_type() -> FileType {
	FileType::Ron
}

pub fn get_config_path(dir_path: &Path, ext: &str) -> PathBuf {
	let mut target = dir_path.to_path_buf();
	target.push("config");
	target.set_extension(ext);
	target
}

pub fn create_config_file(
	config_path: &Path,
	config: &Config,
	file_type: FileType,
) -> Result<File, IOError> {
	match new_empty_file(config_path) {
		Ok(target_file) => {
			if config.to_file(&target_file, file_type).is_ok() {
				Ok(target_file)
			} else {
				Err(IOError::other("configファイルの作成に失敗しました"))
			}
		}
		err => err,
	}
}

pub fn reset_config_file(config_path: &Path, config: &Config) -> Result<File, IOError> {
	match open_file_with_overwrite_mode(config_path) {
		Ok(target_file) => {
			if config.to_file(&target_file, FileType::Ron).is_ok() {
				Ok(target_file)
			} else {
				Err(IOError::other("configファイルの作成に失敗しました"))
			}
		}
		err => err,
	}
}

pub fn find_config_from_dir_path(dir_path: &Path) -> Option<(Config, PathBuf)> {
	for ext in FileType::VARIANTS.iter() {
		let config_path = get_config_path(dir_path, ext);
		if let Ok(true) = &config_path.try_exists() {
			let file_type = FileType::from_str(ext).unwrap(); // 成功しないとおかしいのでunwrap
			let file = open_file_with_read_mode(&config_path);
			if let Ok(f) = file
				&& let Ok(config) = Config::read(&f, file_type)
			{
				return Some((config, config_path));
			}
		}
	}

	let parent = dir_path.parent();
	if let Some(p) = parent {
		find_config_from_dir_path(p)
	} else {
		None
	}
}
