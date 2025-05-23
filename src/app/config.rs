use std::fs::{self, File};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};

use ron::error::SpannedResult;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use super::fs::io::{new_empty_file, open_file_with_overwrite_mode, write_string};
use super::serde::FileType;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	project_name: String,
	site_name: String,
	#[serde(default)]
	dir_conf: DirConf,
}

pub fn default_project_name() -> String {
	String::from("nibi_project")
}

pub fn default_site_name() -> String {
	String::from("nibi_site")
}

impl Default for Config {
	fn default() -> Self {
		Self {
			project_name: default_project_name(),
			site_name: default_site_name(),
			dir_conf: DirConf::default(),
		}
	}
}

impl Config {
	pub fn new(project_name: String, site_name: String) -> Self {
		Self {
			project_name,
			site_name,
			dir_conf: DirConf::default(),
		}
	}

	pub fn project_name<T: Into<String>>(mut self, proj_name: T) -> Self {
		self.project_name = proj_name.into();
		self
	}

	pub fn site_title<T: Into<String>>(mut self, site_name: T) -> Self {
		self.site_name = site_name.into();
		self
	}

	pub fn to_ron(&self, pretty_config: Option<PrettyConfig>) -> String {
		let pretty_config =
			pretty_config.unwrap_or(PrettyConfig::new().depth_limit(3).struct_names(true));
		match ron::ser::to_string_pretty(self, pretty_config) {
			Err(err) => {
				println!("{}", err);
				// デフォルト値の出力失敗は想定外のため
				panic!()
			}
			Ok(ron_string) => ron_string,
		}
	}

	pub fn from_ron(ron_str: &str) -> SpannedResult<Self> {
		ron::from_str(ron_str)
	}

	pub fn from_ron_file(file: File) -> SpannedResult<Config> {
		ron::de::from_reader(file)
	}

	pub fn get_dir_conf(&self) -> &DirConf {
		&self.dir_conf
	}
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DirConf {
	#[serde(default)]
	site: PathBuf, // 出力先
	#[serde(default)]
	zairyo: PathBuf, // 生地データなどの材料
	#[serde(default)]
	igata: PathBuf, // 鋳型
	#[serde(default)]
	gears: PathBuf, //アドオン設定置き予定
}

impl Default for DirConf {
	fn default() -> Self {
		Self {
			site: PathBuf::from(String::from("site")),
			zairyo: PathBuf::from(String::from("zairyo")),
			igata: PathBuf::from(String::from("igata")),
			gears: PathBuf::from(String::from("gears")),
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

pub fn create_config_file(config_path: &Path, config: &Config) -> Result<File, IOError> {
	match new_empty_file(config_path) {
		Ok(target_file) => {
			let serialized_config = config.to_ron(None);
			write_string(target_file, serialized_config)
		}
		err => err,
	}
}

pub fn reset_config_file(config_path: &Path, config: &Config) -> Result<File, IOError> {
	match open_file_with_overwrite_mode(config_path) {
		Ok(target_file) => {
			let serialized_config = config.to_ron(None);
			write_string(target_file, serialized_config)
		}
		err => err,
	}
}
