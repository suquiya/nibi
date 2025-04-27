use std::{
	env::current_dir,
	path::{self, Path, PathBuf},
};

pub fn get_abs_path<T: Into<PathBuf>>(path_str: T) -> PathBuf {
	let path: PathBuf = path_str.into();
	if path.is_absolute() {
		path
	} else {
		path::absolute(path).unwrap()
	}
}

pub fn get_abs_path_from_option<T: Into<PathBuf>>(path_str: Option<T>) -> PathBuf {
	match path_str {
		Some(path) => get_abs_path(path),
		None => current_dir().unwrap(),
	}
}

pub fn file_name(path: &Path) -> String {
	path.file_name().unwrap().to_str().unwrap().to_string()
}
