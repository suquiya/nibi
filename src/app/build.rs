use std::path::Path;

use walkdir::WalkDir;

use crate::app::category::get_categories_from_dir_path;

use super::{
	config::Config,
	tag::{Tag, get_tags_from_dir_path},
};

pub fn build((config, config_path): (Config, &Path), proj_path: &Path) {
	let zairyo_dir = config.get_dir_conf().get_zairyo_path(proj_path);

	let categories = get_categories_from_dir_path(proj_path).unwrap_or_default();

	let tags: Vec<Tag> = get_tags_from_dir_path(proj_path).unwrap_or_default();

	for entry in WalkDir::new(zairyo_dir)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".ingot"))
	{}
}
