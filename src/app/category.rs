use std::{
	collections::BTreeMap,
	path::{Path, PathBuf},
};

use combu::Vector;

use serde::{Deserialize, Serialize};

use super::serde::{DeResult, FileType, read_deserialized_value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
	pub id: usize,
	pub path_name: String,
	pub name: String,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub parent_id: Option<usize>,
	#[serde(skip_serializing_if = "Vector::is_none", default)]
	pub children: Vector<Category>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub opt_attrs: Option<BTreeMap<String, String>>,
}

impl Category {
	pub fn new_with_all(
		id: usize,
		path_name: String,
		name: String,
		description: String,
		parent_id: Option<usize>,
		children: Vector<Category>,
		opt_attrs: Option<BTreeMap<String, String>>,
	) -> Self {
		Self {
			id,
			path_name,
			name,
			description,
			parent_id,
			children,
			opt_attrs,
		}
	}
	pub fn new(id: usize, path_name: String, name: String, description: String) -> Self {
		Self::new_with_all(id, path_name, name, description, None, Vector(None), None)
	}

	pub fn new_with_parent(
		id: usize,
		path_name: String,
		name: String,
		description: String,
		parent_id: Option<usize>,
	) -> Self {
		Self::new_with_all(
			id,
			path_name,
			name,
			description,
			parent_id,
			Vector(None),
			None,
		)
	}

	pub fn append_child(&mut self, child: Category) {
		self.children.push(child);
	}

	/// Returns None if `descendant` is a descendant of `self`.
	pub fn insert_descendant_if_match(&mut self, descendant: Category) -> Option<Category> {
		match descendant.parent_id {
			Some(parent_id) if parent_id == self.id => {
				self.append_child(descendant);
				None
			}
			Some(_) => match &mut self.children {
				Vector(Some(children)) => insert_descendant_to_category_list(children, descendant),
				Vector(None) => Some(descendant),
			},
			None => Some(descendant),
		}
	}

	pub fn exists_id(&self, id: usize) -> bool {
		if self.id == id {
			true
		} else if let Vector(Some(children)) = &self.children {
			children.iter().any(|child| child.exists_id(id))
		} else {
			false
		}
	}

	pub fn search_id(&self, id: usize) -> Option<&Category> {
		if self.id == id {
			Some(self)
		} else if let Vector(Some(children)) = &self.children {
			children.iter().find_map(|child| child.search_id(id))
		} else {
			None
		}
	}

	pub fn get_descendants(&self) -> Vec<&Category> {
		let mut descendants = Vec::new();
		descendants.push(self);
		if let Vector(Some(children)) = &self.children {
			for child in children.iter() {
				descendants.append(&mut child.get_descendants());
			}
		}
		descendants
	}
}

pub fn insert_descendant_to_category_list(
	list: &mut [Category],
	descendant: Category,
) -> Option<Category> {
	let mut des = descendant;
	for category in list.iter_mut() {
		match category.insert_descendant_if_match(des) {
			Some(r) => {
				des = r;
			}
			None => return None,
		}
	}
	Some(des)
}

pub fn search_id_in_category_list(list: &[Category], id: usize) -> Option<&Category> {
	list.iter().find_map(|category| category.search_id(id))
}

pub fn exists_id_in_category_list(list: &[Category], id: usize) -> bool {
	list.iter().any(|category| category.exists_id(id))
}

fn categories_file_path(dir_path: &Path) -> PathBuf {
	dir_path.join("categories.ron")
}
fn read_categories<R: std::io::Read>(reader: R, file_type: FileType) -> DeResult<Vec<Category>> {
	read_deserialized_value(reader, file_type)
}

pub fn get_categories_from_dir_path(dir_path: &Path) -> Option<Vec<Category>> {
	let file_path = categories_file_path(dir_path);
	let file = std::fs::File::open(file_path).ok()?;
	read_categories(file, FileType::Ron).ok()
}

pub fn get_index_map_from_categories(categories: &[Category]) -> BTreeMap<usize, &Category> {
	let mut map = BTreeMap::new();
	for category in categories.iter() {
		let list = category.get_descendants();
		for category in list.into_iter() {
			map.insert(category.id, category);
		}
	}
	map
}
