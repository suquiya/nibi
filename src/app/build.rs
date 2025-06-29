use std::path::Path;

use walkdir::WalkDir;

use crate::app::{
	category::{get_categories_from_dir_path, get_index_map_from_categories},
	fs::io::open_file_with_read_mode,
	ingot::Ingot,
	tag::get_index_map_from_tags,
};

use super::{
	config::Config,
	tag::{Tag, get_tags_from_dir_path},
};

pub fn build((config, _config_path): (Config, &Path), proj_path: &Path) {
	let zairyo_dir = config.get_dir_conf().get_zairyo_path(proj_path);

	let categories = get_categories_from_dir_path(&zairyo_dir).unwrap_or_default();

	let tags: Vec<Tag> = get_tags_from_dir_path(&zairyo_dir).unwrap_or_default();

	let mut ingots: Vec<Ingot> = Vec::new();

	let index_categories_map = get_index_map_from_categories(&categories);
	let index_tags_map = get_index_map_from_tags(&tags);

	for entry in WalkDir::new(zairyo_dir)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".ingot"))
	{
		let reader = open_file_with_read_mode(entry.path()).unwrap();
		match Ingot::read(reader) {
			Ok(mut ingot) => {
				// ingotのカテゴリとタグを照合
				ingot.collate_ids(&index_categories_map, &index_tags_map);

				ingots.push(ingot);
			}
			Err(e) => {
				println!("{}: {}", entry.path().display(), e);
			}
		}
	}
}
