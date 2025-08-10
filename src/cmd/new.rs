use combu::{Command, Context, Flag, action_result, alias, done, flags, license, vector};

use crate::{
	app::{config::find_config_from_dir_path, fs::path::to_parent_abs_path, igata::create_new_pack},
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
		"igata_pack".to_owned(),
		Some(route_common!(new_igt_pack_action)),
		String::default(),
		String::default(),
		license![],
		Some("create new igata pack (, or theme).".to_owned()),
		"nibi new igata_pack [igata set's name]".to_owned(),
		vector![project_dir_flag()],
		vector![],
		new_igt_pack_alias().into(),
		String::default(),
		vector![sub_help()],
	)
}

pub fn not_specified_target_action(_cmd: Command, _ctx: Context) -> action_result!() {
	println!("specify new target: 新しく作成するものを指定してください。");
	println!("now available target: ");
	println!("\t + igata_pack ({})", new_igt_pack_alias().join("/"));
	done!()
}

fn new_igt_pack_alias() -> Vec<String> {
	vec![
		"theme".to_owned(),
		"igt_pack".to_owned(),
		"igata_set".to_owned(),
		"igata-set".to_owned(),
		"igata_tsuduri".to_owned(),
		"igata-tuduri".to_owned(),
		"igata_tuduri".to_owned(),
		"igata-tuduri".to_owned(),
	]
}

pub fn new_igt_pack_action(_cmd: Command, ctx: Context) -> action_result!() {
	if let Some(igata_pack_name) = ctx.args.front() {
		let proj_dir = get_proj_dir_from_context(&ctx);
		let (config, config_path) = get_config_common!(proj_dir);

		let proj_path = to_parent_abs_path(config_path);

		create_new_pack(
			&config.get_dir_conf().get_igata_path(&proj_path),
			igata_pack_name.trim().to_string(),
		);
	} else {
		println!("specify theme name: テーマ名を指定してください。");
	}
	done!()
}

pub fn new_recipe_cmd() -> Command {
	Command::with_all_field(
		"recipe".into(),
		Some(route_common!(new_recipe_action)),
		String::default(),
		String::default(),
		license![],
		Some("create new recipe".to_owned()),
		"nibi new recipe [recipe name]".to_owned(),
		flags![],
		vector![],
		vector![],
		String::default(),
		vector![sub_help()],
	)
}

pub fn new_recipe_action(_cmd: Command, ctx: Context) -> action_result!() {
	if let Some(recipe_name) = ctx.args.front() {
		let proj_dir = get_proj_dir_from_context(&ctx);
		let (config, config_path) = get_config_common!(proj_dir);

		let proj_path = to_parent_abs_path(config_path);
	}
	done!()
}
