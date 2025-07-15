use combu::{Command, Context, Flag, Vector, action_result, done, flags, license, vector};

use crate::{
	app::{build::build, config::find_config_from_dir_path, fs::path::get_abs_path_from_option},
	get_config_common, route_common,
};

use super::common::sub_help;

pub fn cmd() -> Command {
	Command::with_all_field(
		"build".to_owned(),
		Some(route_common!(build_action)),
		String::default(),
		String::default(),
		license![],
		Some("build nibi project".to_owned()),
		"nibi build [directory path: default is current]".to_owned(),
		flags(),
		flags![],
		vector![],
		String::default(),
		vector![sub_help()],
	)
}

pub fn flags() -> Vector<Flag> {
	vector![]
}

pub fn build_action(_cmd: Command, ctx: Context) -> action_result!() {
	let proj_path = get_abs_path_from_option(ctx.args.front());
	println!("dir_path: {}", proj_path.display());
	// 存在しているディレクトリか確認
	if !proj_path.is_dir() {
		println!("{} is not directory or does not exist", proj_path.display());
		return done!();
	}
	let proj_path = proj_path.to_path_buf();

	// configを取得
	let (config, config_path) = get_config_common!(proj_path);
	// config_pathからプロジェクトパスを修正
	let proj_path = config_path.parent().unwrap().to_path_buf();

	build(config, &proj_path);

	done!()
}
