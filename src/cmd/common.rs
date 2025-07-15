use std::{env::current_dir, path::PathBuf};

use combu::{
	Command, Context, Flag, FlagType, FlagValue, action::bundle::Bundle, action_result,
	command::presets::func::help_tablize_with_alias_dedup, preset_help_command, vector,
};

use crate::cli;

pub fn sub_help() -> Command {
	preset_help_command!(help_tablize_with_alias_dedup)
}

pub fn get_flagged_yes_no(bundle: &Bundle) -> Option<bool> {
	if bundle
		.get_local_flag_value_of("no")
		.unwrap()
		.get_bool_unwrap()
	{
		Some(false)
	} else if bundle
		.get_local_flag_value_of("yes")
		.unwrap()
		.get_bool_unwrap()
	{
		Some(true)
	} else {
		None
	}
}

pub fn overwrite_confirm(yes_no_flag: Option<bool>) -> Option<bool> {
	get_yes_no(yes_no_flag, "上書きしますか？")
}

pub fn get_yes_no(yes_no_flag: Option<bool>, message: &str) -> Option<bool> {
	match yes_no_flag {
		None => cli::prompt::yes_or_no(message),
		_ => yes_no_flag,
	}
}

pub fn get_yes_no_with_default(yes_no_flag: Option<bool>, message: &str, default: bool) -> bool {
	match yes_no_flag {
		None => cli::prompt::yes_or_no_with_default(message, default),
		Some(f) => f,
	}
}

pub fn take_to_string_option(bundle: &mut Bundle, flag_name: &str) -> Option<String> {
	match bundle.take_inputted_flag_value_of(flag_name) {
		Some(FlagValue::String(s)) => Some(s),
		_ => None,
	}
}

pub fn take_to_bool_option(bundle: &mut Bundle, flag_name: &str) -> Option<bool> {
	match bundle.take_inputted_flag_value_of(flag_name) {
		Some(FlagValue::Bool(b)) => Some(b),
		_ => None,
	}
}

pub fn project_dir_flag() -> Flag {
	Flag::with_all_field(
		"project-dir".to_owned(),
		"specify project directory".to_owned(),
		vector!['d', 'c'],
		vector!["proj-dir", "pj-d";=>String],
		FlagType::String,
		FlagValue::from(""),
	)
}

pub fn get_proj_dir_from_context(ctx: &Context) -> PathBuf {
	match ctx.get_inputted_local_flag_value_of("project-dir") {
		Some(FlagValue::String(s)) => PathBuf::from(s),
		_ => current_dir().unwrap(),
	}
}

#[macro_export]
macro_rules! route_common {
	($action:ident) => {
		|mut cmd: Command, ctx: Context| -> action_result!() {
			combu::checks!(cmd, ctx, [error, help, version, license]);
			cmd.action = Some($action);
			Ok(combu::ActionResult::Result(cmd, ctx))
		}
	};
}

#[macro_export]
macro_rules! nibi_copyright {
	() => {
		copyright!(2024, suquiya)
	};
}

#[macro_export]
macro_rules! get_config_common {
	($dir: ident) => {
		match find_config_from_dir_path(&$dir) {
			Some(c) => c,
			_ => {
				println!("config not found, please run `nibi init`");
				return done!();
			}
		}
	};
}
