use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn get_config_path(dir_path: &Path, ext: &str) -> PathBuf {
    let mut target = dir_path.to_path_buf();
    target.push("config");
    target.set_extension(ext);
    return target;
}
pub fn create_config_file(config_path: &Path) -> Result<File, Error> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_path)
    {
        Ok(mut target_file) => {
            println!("OK!");
            let message = "test";
            let result = write!(target_file, "{}", message);
            match result {
                Ok(()) => {
                    println!("書き込みに成功しました");
                    return Ok(target_file);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(err);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    }
}

pub fn overwrite_config_file(config_path: &Path) -> Result<File, Error> {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_path)
    {
        Ok(mut target_file) => {
            let message = "overwrite test";
            match write!(target_file, "{}", message) {
                Ok(()) => {
                    println!("上書きしました");
                    return Ok(target_file);
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(err);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    }
}
