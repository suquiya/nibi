use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app::{
	config::Config,
	fs::{
		io::{new_empty_file, open_file_with_read_mode},
		path::append_ext,
	},
	serde::{
		DeError, DeResult, FileType, StrValOrArray, read_deserialized_value,
		write_serialized_string_all,
	},
};
/// Recipe struct. Holds pack list, igata table, and values for building a site.
pub struct Recipe {
	/// List of pack names to build a site.
	pub pack: Vec<String>,
	/// Igata table for template rendering.
	pub igata_table: BTreeMap<String, String>,
	/// Values for template rendering.
	pub values: BTreeMap<String, String>,
}
/// Returns the default igata table.
pub fn default_igata_table() -> BTreeMap<String, String> {
	["index", "post", "page", "list", "index"]
		.iter()
		.map(|s| (s.to_string(), s.to_string()))
		.collect()
}

/// Returns the default values for template rendering.
pub fn default_values(site_name: &str) -> BTreeMap<String, String> {
	let mut values = BTreeMap::new();
	values.insert("site_name".to_string(), site_name.to_string());
	values
}

impl Recipe {
	/// Creates a new `Recipe` with all fields.
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

	/// Creates a new `Recipe` with the given config and settings.
	pub fn new(config: &Config, settings: RecipeSettings) -> Self {
		let (pack, overrides) = settings.take_fields();
		let mut igata_table = default_igata_table();
		igata_table.extend(overrides.igata_table);
		let mut values = default_values(config.site_name_ref());
		values.extend(overrides.values);
		Self::new_with_all_fields(pack, igata_table, values)
	}

	/// Returns the pack names for this recipe.
	pub fn get_pack_names(&self) -> &[String] {
		&self.pack
	}
	/// Returns the pack names for this recipe, deduplicated. The order is not preserved.
	pub fn get_pack_names_dedup(&self) -> Vec<String> {
		let mut vec: Vec<String> = self.pack.clone();
		vec.sort();
		vec.dedup();
		vec
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
/// Overrides for the igata table and values.
pub struct Overrides {
	#[serde(default)]
	/// Igata table overrides.
	pub igata_table: BTreeMap<String, String>,
	#[serde(default)]
	/// Value overrides.
	pub values: BTreeMap<String, String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
/// Settings for a recipe, including the pack and overrides. Used to users' configuration.
pub struct RecipeSettings {
	/// The pack names for this recipe.
	pub pack: StrValOrArray,
	/// Overrides for the igata table and values.
	pub overrides: Overrides,
}

impl RecipeSettings {
	/// Creates a new `RecipeSettings` with the given pack and overrides.
	pub fn new(pack: Vec<String>, overrides: Overrides) -> Self {
		Self {
			pack: StrValOrArray(pack),
			overrides,
		}
	}

	/// Returns a reference to the pack names for this recipe.
	pub fn get_pack(&self) -> &Vec<String> {
		self.pack.inner()
	}

	/// Takes ownership of the pack names for this recipe.
	pub fn take_pack(self) -> Vec<String> {
		self.pack.take_inner()
	}

	/// Returns a reference to the overrides for this recipe.
	pub fn get_overrides(&self) -> &Overrides {
		&self.overrides
	}

	/// Takes ownership of the overrides for this recipe.
	pub fn take_overrides(self) -> Overrides {
		self.overrides
	}

	/// Takes ownership of both the pack names and overrides for this recipe.
	pub fn take_fields(self) -> (Vec<String>, Overrides) {
		let (pack, overrides) = (self.pack, self.overrides);
		(pack.take_inner(), overrides)
	}
}

/// Normalizes a recipe name by removing whitespace and invalid characters.
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
/// Creates a new recipe file with the given name in the project directory.
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
/// Reads the recipe file for the current project and returns a `Recipe` struct.
pub fn read_recipe(config: &Config, proj_dir_path: &Path) -> DeResult<Recipe> {
	let recipe_path = get_recipe_path(proj_dir_path, config.get_recipe().clone());
	match open_file_with_read_mode(&recipe_path) {
		Ok(file) => read_deserialized_value(file, FileType::Ron)
			.map(|settings: RecipeSettings| Recipe::new(config, settings)),
		Err(e) => {
			println!("failed to read recipe file: レシピファイルの読み込みに失敗しました - {e}");
			Err(DeError::IO(e))
		}
	}
}

fn get_recipe_path(proj_dir_path: &Path, recipe_name: String) -> PathBuf {
	let recipe_name = norm_recipe_name(recipe_name);
	append_ext(proj_dir_path.join(&recipe_name), "ron")
}
