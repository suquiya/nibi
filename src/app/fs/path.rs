use std::{
	collections::BTreeMap,
	env::current_dir,
	path::{self, Path, PathBuf},
};
/// Returns the absolute path of the given path string.
pub fn get_abs_path<T: Into<PathBuf>>(path_str: T) -> PathBuf {
	let path: PathBuf = path_str.into();
	if path.is_absolute() {
		path
	} else {
		path::absolute(path).unwrap()
	}
}

/// Returns the absolute path of the given path string, or the current directory if `None`.
pub fn get_abs_path_from_option<T: Into<PathBuf>>(path_str: Option<T>) -> PathBuf {
	match path_str {
		Some(path) => get_abs_path(path),
		None => current_dir().unwrap(),
	}
}

/// Returns the file name of the given path.
pub fn file_name(path: &Path) -> Option<String> {
	path
		.file_name()
		.and_then(|n| n.to_str())
		.map(|s| s.to_string())
}

/// Returns the directory path of the given path as a string.
pub fn get_dir_path_string(path: &Path) -> String {
	match path.to_str() {
		Some(val) => val.to_string(),
		None => path.to_string_lossy().to_string(),
	}
}

/// Appends the given extension to the path.
pub fn append_ext(path: PathBuf, ext: &str) -> PathBuf {
	let mut path = path;
	path.set_extension(ext);
	path
}

/// Returns the parent path of the given path.
pub fn to_parent_path(path: PathBuf) -> PathBuf {
	path.parent().unwrap().to_path_buf()
}

/// Returns the parent path of the given path after resolve to an absolute path.
pub fn to_parent_abs_path(path: PathBuf) -> PathBuf {
	to_parent_path(path).canonicalize().unwrap()
}

/// Returns a map of file names to paths from the given vector of paths.
pub fn to_path_map(path: Vec<PathBuf>) -> BTreeMap<String, PathBuf> {
	let mut map = BTreeMap::new();
	for p in path {
		if let Some(name) = file_name(&p) {
			map.insert(name, p);
		}
	}
	map
}
