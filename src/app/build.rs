use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::app::{
	category::{get_categories_from_dir_path, get_index_map_from_categories},
	fs::io::open_file_with_read_mode,
	igata::pack::get_packs_from_names,
	ingot::Ingot,
	recipe::read_recipe,
	tag::get_index_map_from_tags,
};

use super::{
	config::Config,
	tag::{Tag, get_tags_from_dir_path},
};
/// Builds the website of the project.
pub fn build(config: Config, proj_path: &Path) {
	let zairyo_dir = config.get_dir_conf().get_zairyo_path(proj_path);

	let categories = get_categories_from_dir_path(&zairyo_dir).unwrap_or_default();

	let tags: Vec<Tag> = get_tags_from_dir_path(&zairyo_dir).unwrap_or_default();

	let mut ingots: BTreeMap<usize, (PathBuf, Ingot)> = BTreeMap::new();

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

				ingots.insert(ingot.id, (entry.path().to_path_buf(), ingot));
			}
			Err(e) => {
				println!("{}: {}", entry.path().display(), e);
			}
		}
	}

	// レシピを読む
	let recipe = match read_recipe(&config, proj_path) {
		Ok(recipe) => recipe,
		Err(e) => {
			println!("Failed to read recipe: {}", e);
			return;
		}
	};

	// 必要なpackのデータを読み込んでおく
	let _packs = get_packs_from_names(
		recipe.get_pack_names(),
		&config.get_dir_conf().get_igata_path(proj_path),
	);
}
