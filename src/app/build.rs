use std::path::Path;

use walkdir::WalkDir;

use crate::app::{
	category::get_categories_from_dir_path, fs::io::open_file_with_read_mode, ingot::Ingot,
};

use super::{
	config::Config,
	tag::{Tag, get_tags_from_dir_path},
};

pub fn build((config, config_path): (Config, &Path), proj_path: &Path) {
	let zairyo_dir = config.get_dir_conf().get_zairyo_path(proj_path);

	let categories = get_categories_from_dir_path(proj_path).unwrap_or_default();

	let tags: Vec<Tag> = get_tags_from_dir_path(proj_path).unwrap_or_default();

	let mut i = 0;
	for entry in WalkDir::new(zairyo_dir)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".ingot"))
	{
		if i < 1 {
			println!("{}: {}", i, entry.path().display());
			let reader = open_file_with_read_mode(entry.path()).unwrap();
			let ingot = Ingot::parse(reader);
			// println!("{:?}", ingot);
		}

		i += 1;
	}
}
