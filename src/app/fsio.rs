use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

pub fn new_empty_file(path: &Path) -> Result<File, Error> {
	match OpenOptions::new().write(true).create_new(true).open(path) {
		Err(err) => {
			println!("{}", err);
			Err(err)
		}
		val => val,
	}
}

pub fn open_file_with_overwrite_mode(path: &Path) -> Result<File, Error> {
	match OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(path)
	{
		Err(err) => {
			println!("{}", err);
			Err(err)
		}
		val => val,
	}
}

pub fn new_file_with_init_contents(path: &Path, init_contents: &str) -> Result<File, Error> {
	match new_empty_file(path) {
		Ok(target_file) => match write_str(target_file, init_contents) {
			Err(err) => {
				println!("{}", err);
				return Err(err);
			}
			ok => ok,
		},
		Err(err) => {
			println!("{}", err);
			return Err(err);
		}
	}
}

pub fn write_str(mut file: File, str: &str) -> Result<File, Error> {
	match file.write_all(str.as_bytes()) {
		Err(err) => {
			println!("{}", err);
			return Err(err);
		}
		_ => Ok(file),
	}
}

pub fn write_string(file: File, string: String) -> Result<File, Error> {
	write_str(file, &string)
}
