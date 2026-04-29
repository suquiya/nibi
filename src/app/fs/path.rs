use std::{
	collections::BTreeMap,
	env::current_dir,
	path::{self, Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};
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

/// Returns a vector of paths for the child directories of the given directory.
pub fn get_child_dirs(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
	dir.read_dir().map(|entries| {
		entries
			.filter_map(|entry| {
				let path = entry.ok()?.path();
				if path.is_dir() { Some(path) } else { None }
			})
			.collect()
	})
}

/// Returns an iterator of `DirEntry` for the all paths in the given directory.
pub fn iter_all_paths(root: &Path) -> impl Iterator<Item = DirEntry> {
	WalkDir::new(root).into_iter().filter_map(|e| e.ok())
}

/// Returns true if `DirEntry` is a directory.
pub fn is_dir(entry: &DirEntry) -> bool {
	entry.file_type().is_dir()
}

/// Returns true if `DirEntry` is a file.
pub fn is_file(entry: &DirEntry) -> bool {
	entry.file_type().is_file()
}

/// Returns true if file and ends with the given extension.
pub fn is_file_ends_with(entry: &DirEntry, ext: &str) -> bool {
	entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(ext)
}
