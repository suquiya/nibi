use std::collections::BTreeMap;

use combu::Vector;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
	pub id: usize,
	pub path_name: String,
	pub name: String,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent_id: Option<usize>,
	#[serde(skip_serializing_if = "Vector::is_none")]
	pub children: Vector<Category>,
	#[serde(skip_serializing_if = "Option::is_none")]
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
