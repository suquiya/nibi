use std::path::Path;

use crate::app::{
	category::{get_categories_from_dir_path, get_index_map_from_categories},
	igata::pack::get_packs_from_names,
	ingot::ingot::get_ingots_from_dir_and_collate_id_maps,
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

	let categories_index_map = get_index_map_from_categories(&categories);
	let tags_index_map = get_index_map_from_tags(&tags);

	let ingots =
		get_ingots_from_dir_and_collate_id_maps(&zairyo_dir, &categories_index_map, &tags_index_map);

	// レシピを読む
	let recipe = read_recipe(&config, proj_path).unwrap();

	// 必要なpackのデータを読み込んでおく
	let _packs = get_packs_from_names(
		recipe.get_pack_names(),
		&config.get_dir_conf().get_igata_path(proj_path),
	);
}
