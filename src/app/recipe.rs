use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app::{
	config::Config,
	fs::{io::new_empty_file, path::append_ext},
	serde::{FileType, StrValOrArray, write_serialized_string_all},
};

pub struct Recipe {
	pub pack: Vec<String>,
	pub igata_table: BTreeMap<String, String>,
	pub values: BTreeMap<String, String>,
}

pub fn default_igata_table() -> BTreeMap<String, String> {
	["index", "post", "page", "list", "index"]
		.iter()
		.map(|s| (s.to_string(), s.to_string()))
		.collect()
}

pub fn default_values(site_name: &str) -> BTreeMap<String, String> {
	let mut values = BTreeMap::new();
	values.insert("site_name".to_string(), site_name.to_string());
	values
}

impl Recipe {
	pub fn new_with_all_fields(
		pack: Vec<String>,
		igata_table: BTreeMap<String, String>,
		values: BTreeMap<String, String>,
	) -> Self {
		Self {
			pack,
			igata_table,
			values,
		}
	}

	pub fn new(config: &Config, settings: RecipeSettings) -> Self {
		let (pack, overrides) = settings.take_fields();
		let mut igata_table = default_igata_table();
		igata_table.extend(overrides.igata_table);
		let mut values = default_values(&config.site_name_ref());
		values.extend(overrides.values);
		Self::new_with_all_fields(pack, igata_table, values)
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Overrides {
	#[serde(default)]
	pub igata_table: BTreeMap<String, String>,
	#[serde(default)]
	pub values: BTreeMap<String, String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RecipeSettings {
	pub pack: StrValOrArray,
	pub overrides: Overrides,
}

impl RecipeSettings {
	pub fn new(pack: Vec<String>, overrides: Overrides) -> Self {
		Self {
			pack: StrValOrArray(pack),
			overrides,
		}
	}

	pub fn get_pack(&self) -> &Vec<String> {
		self.pack.inner()
	}

	pub fn take_pack(self) -> Vec<String> {
		self.pack.take_inner()
	}

	pub fn get_overrides(&self) -> &Overrides {
		&self.overrides
	}

	pub fn take_overrides(self) -> Overrides {
		self.overrides
	}

	pub fn take_fields(self) -> (Vec<String>, Overrides) {
		let (pack, overrides) = (self.pack, self.overrides);
		(pack.take_inner(), overrides)
	}
}

pub fn norm_recipe_name(recipe_name: String) -> String {
	recipe_name
		.trim()
		.chars()
		.filter(|c| {
			!c.is_whitespace()
				&& (c.is_alphanumeric() || ['-', '_', '@', '#', '+', '.', ':'].contains(c))
		})
		.collect()
}

pub fn create_new_recipe(proj_dir_path: &Path, recipe_name: String) {
	let recipe_name = norm_recipe_name(recipe_name);
	println!("begin {recipe_name} creation: {recipe_name}の作成を開始します");

	let recipe_path = append_ext(proj_dir_path.join(&recipe_name), "ron");

	match new_empty_file(&recipe_path) {
		Ok(file) => {
			println!(
				"create new recipe file {0}: 新しいレシピファイルを作成しました {0}",
				recipe_path.display()
			);
			let _ = write_serialized_string_all(
				file,
				&RecipeSettings::new(vec!["default".to_string()], Overrides::default()),
				FileType::Ron,
			);
		}
		Err(e) => {
			println!("failed to create new recipe file: レシピファイルの作成に失敗しました - {e}");
		}
	}
}

pub fn get_recipe_path(proj_dir_path: &Path, recipe_name: String) -> PathBuf {
	let recipe_name = norm_recipe_name(recipe_name);
	append_ext(proj_dir_path.join(&recipe_name), "ron")
}
