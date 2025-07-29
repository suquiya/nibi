use combu::{Command, Context, Flag, action_result, alias, done, flags, license, vector};

use crate::{
	app::{config::find_config_from_dir_path, igata::create_new_pack},
	cmd::common::{get_proj_dir_from_context, project_dir_flag, sub_help},
	get_config_common, route_common,
};

pub fn threed_cmd() -> Command {
	Command::with_all_field(
		"new".to_owned(),
		Some(route_common!(not_specified_target_action)),
		String::default(),
		String::default(),
		license![],
		Some("create new [site/project|ingot/post|igata/template|igata_set/theme]".to_owned()),
		"nibi new [site/project|ingot/post|igata/template|theme]".to_owned(),
		flags![],
		vector![],
		alias!["n", "create"],
		String::default(),
		vector![sub_help(), new_igt_pack_cmd()],
	)
}

pub fn new_igt_pack_cmd() -> Command {
	Command::with_all_field(
		"igata_set".to_owned(),
		Some(route_common!(new_theme_action)),
		String::default(),
		String::default(),
		license![],
		Some("create new igata pack (, or theme).".to_owned()),
		"nibi new igata_pack [igata set's name]".to_owned(),
		vector![project_dir_flag()],
		vector![],
		vector!["theme","igata_set","igata-set", "igata_tsuduri","igata-tuduri", "igata_tuduri", "igata-tuduri" ;=>String],
		String::default(),
		vector![sub_help()],
	)
}

pub fn not_specified_target_action(_cmd: Command, _ctx: Context) -> action_result!() {
	println!("specify new target: 新しく作成するものを指定してください。");
	println!("now available target: [igata_set(igata_tuduri/igata_tsuduri/theme)]");
	done!()
}

pub fn new_theme_action(_cmd: Command, ctx: Context) -> action_result!() {
	if let Some(igata_pack_name) = ctx.args.front() {
		let proj_dir = get_proj_dir_from_context(&ctx);
		let (config, config_path) = get_config_common!(proj_dir);

		let proj_path = config_path
			.parent()
			.unwrap()
			.to_path_buf()
			.canonicalize()
			.unwrap();

		create_new_pack(
			&config.get_dir_conf().get_igata_path(&proj_path),
			igata_pack_name.trim().to_string(),
		);
	} else {
		println!("specify theme name: テーマ名を指定してください。");
	}
	done!()
}
