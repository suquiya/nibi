use std::fs::{self, File, OpenOptions};
use std::io::{Error, Read, Write};
use std::path::Path;

pub fn new_empty_file(path: &Path) -> Result<File, Error> {
	OpenOptions::new().write(true).create_new(true).open(path)
}

pub fn open_file_with_overwrite_mode(path: &Path) -> Result<File, Error> {
	OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(path)
}

pub fn open_file_with_append_mode(path: &Path) -> Result<File, Error> {
	OpenOptions::new().append(true).create(true).open(path)
}

pub fn open_file_with_read_mode(path: &Path) -> Result<File, Error> {
	OpenOptions::new().read(true).open(path)
}

pub fn new_file_with_init_contents(path: &Path, init_contents: &str) -> Result<File, Error> {
	match new_empty_file(path) {
		Ok(target_file) => write_str(target_file, init_contents),
		val => val,
	}
}

pub fn write_str(mut file: File, str: &str) -> Result<File, Error> {
	match file.write_all(str.as_bytes()) {
		Ok(()) => Ok(file),
		Err(err) => Err(err),
	}
}

pub fn write_string(file: File, string: String) -> Result<File, Error> {
	write_str(file, &string)
}

pub fn read_all(path: &Path) -> Result<String, Error> {
	fs::read_to_string(path)
}

pub fn read_all_from_reader<R: Read>(mut reader: R) -> Result<String, Error> {
	let mut buf = String::new();
	match reader.read_to_string(&mut buf) {
		Ok(_) => Ok(buf),
		Err(err) => Err(err),
	}
}

pub fn read_all_byte(mut file: &File) -> Result<(usize, Vec<u8>), Error> {
	let mut buf = Vec::new();
	match file.read_to_end(&mut buf) {
		Ok(buf_size) => Ok((buf_size, buf)),
		Err(err) => Err(err),
	}
}
