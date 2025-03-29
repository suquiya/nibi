use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OptAttrValue {
	Bool(bool),
	String(String),
	Int(isize),
	Float(f64),
	UInt(usize),
	Char(char),
	Array(Vec<OptAttrValue>),
	None,
	Map(BTreeMap<String, OptAttrValue>),
}

impl Default for OptAttrValue {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Default)]
pub struct OptAttrs(pub Option<BTreeMap<String, OptAttrValue>>);

impl OptAttrs {
	/// Creates new OptAttrs
	pub fn new() -> Self {
		Self::default()
	}
	/// Creates new OptAttrs
	pub fn new_with(opt: BTreeMap<String, OptAttrValue>) -> Self {
		Self(Some(opt))
	}
	/// Returns inner Option
	pub fn inner(&mut self) -> &mut Option<BTreeMap<String, OptAttrValue>> {
		&mut self.0
	}
	/// Returns attr value
	pub fn get(&self, key: &str) -> Option<&OptAttrValue> {
		match &self.0 {
			Some(inner) => inner.get(key),
			_ => None,
		}
	}

	/// Returns true if attrs contains key
	pub fn has_key(&self, key: &str) -> bool {
		match &self.0 {
			Some(inner) => inner.contains_key(key),
			_ => false,
		}
	}

	/// Inserts a key-value pair into `OptAttrs` and returns the previous value associated with the key. If `OptAttrs` is empty, it will be initialized with a new `BTreeMap`.
	pub fn insert(&mut self, key: String, value: OptAttrValue) -> Option<OptAttrValue> {
		match &mut self.0 {
			Some(inner) => inner.insert(key, value),
			_ => {
				let mut inner = BTreeMap::new();
				let r = inner.insert(key, value);
				self.0 = Some(inner);
				r
			}
		}
	}

	/// Removes a key-value pair from `OptAttrs` and returns the previous value associated with the key.
	/// If `OptAttrs` is empty, it will return `None`.
	pub fn remove(&mut self, key: &str) -> Option<OptAttrValue> {
		match &mut self.0 {
			Some(inner) => inner.remove(key),
			_ => None,
		}
	}

	/// Clears `OptAttrs`'s inner.
	pub fn clear(&mut self) {
		if let Some(inner) = &mut self.0 {
			inner.clear()
		}
	}
}

impl From<Option<BTreeMap<String, OptAttrValue>>> for OptAttrs {
	fn from(opt: Option<BTreeMap<String, OptAttrValue>>) -> Self {
		Self(opt)
	}
}

impl From<BTreeMap<String, OptAttrValue>> for OptAttrs {
	fn from(opt: BTreeMap<String, OptAttrValue>) -> Self {
		OptAttrs::new_with(opt)
	}
}
