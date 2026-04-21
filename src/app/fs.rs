use std::path::{Path, PathBuf};

/// io utility module
pub mod io;
/// path unitity module
pub mod path;

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
