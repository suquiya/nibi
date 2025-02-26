use std::{env::current_dir, path::PathBuf};

pub fn get_path_from_str(path_str: &str) -> PathBuf {
	let path = PathBuf::from(path_str);
	if path.is_absolute() {
		path
	} else {
		let p = current_dir().unwrap().join(path);
		p
	}
}
