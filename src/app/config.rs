use std::path::PathBuf;
pub fn create_config_file(target: &str) {
    println!("{}", target);
    let mut target = PathBuf::from(target);
    target.push("config");
    target.set_extension("ron");
}
