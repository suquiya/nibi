use std::fs::File;
use std::io::Error as IOError;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::fsio::{new_empty_file, open_file_with_overwrite_mode, write_str};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	project_name: String,
	site_title: String,
	dir_conf: DirConf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DirConf {
	opus_mold: PathBuf,
	ingots: PathBuf,
	igata: PathBuf,
	gears: PathBuf,
}

impl Default for DirConf {
	fn default() -> Self {
		Self {
			opus_mold: PathBuf::default(),
			ingots: PathBuf::default(),
			igata: PathBuf::default(),
			gears: PathBuf::default(),
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			project_name: String::from("nibi_project"),
			site_title: String::from("site_title"),
			dir_conf: DirConf::default(),
		}
	}
}

impl Config {
	pub fn new(project_name: String, site_title: String) -> Self {
		Self {
			project_name,
			site_title,
			dir_conf: DirConf::default(),
		}
	}
}

pub fn get_config_path(dir_path: &Path, ext: &str) -> PathBuf {
	let mut target = dir_path.to_path_buf();
	target.push("config");
	target.set_extension(ext);
	return target;
}

pub fn create_config_file(config_path: &Path) -> Result<File, IOError> {
	match new_empty_file(config_path) {
		Ok(target_file) => {
			let message = "test";
			write_str(target_file, message)
		}
		err => {
			return err;
		}
	}
}

pub fn overwrite_config_file(config_path: &Path) -> Result<File, IOError> {
	match open_file_with_overwrite_mode(config_path) {
		Ok(target_file) => {
			let config = Config::default();
			match ron::to_string(&config) {
				Err(err) => {
					println!("{}", err);
					panic!()
				}
				Ok(serialized_config) => match write_str(target_file, &serialized_config) {
					Err(err) => {
						println!("{}", err);
						return Err(err);
					}
					ok => {
						println!("configファイルを上書きしました");
						return ok;
					}
				},
			}
		}
		err => {
			return err;
		}
	}
}
