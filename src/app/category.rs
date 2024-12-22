use std::collections::BTreeMap;

use combu::Vector;
use ron::de;
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
	pub ex: Option<BTreeMap<String, String>>,
}

impl Category {
	pub fn new_with_all(
		id: usize,
		path_name: String,
		name: String,
		description: String,
		parent_id: Option<usize>,
		children: Vector<Category>,
		ex: Option<BTreeMap<String, String>>,
	) -> Self {
		Self {
			id,
			path_name,
			name,
			description,
			parent_id,
			children,
			ex,
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
				Vector(Some(children)) => {
					let mut des = descendant;
					for child in children.iter_mut() {
						match child.insert_descendant_if_match(des) {
							Some(r) => {
								des = r;
							}
							None => return None,
						}
					}
					Some(des)
				}
				Vector(None) => Some(descendant),
			},
			None => Some(descendant),
		}
	}
}
