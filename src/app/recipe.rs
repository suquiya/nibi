use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::app::serde::StrValOrArray;

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
