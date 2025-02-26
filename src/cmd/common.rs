use combu::{
	action::bundle::Bundle, action_result, command::presets::func::help_tablize_with_alias_dedup,
	preset_help_command, Command, FlagValue,
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
	} else {
		if bundle
			.get_local_flag_value_of("yes")
			.unwrap()
			.get_bool_unwrap()
		{
			Some(true)
		} else {
			None
		}
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

pub fn take_to_string_option(bundle: &mut Bundle, flag_name: &str) -> Option<String> {
	match bundle.take_inputted_local_flag_value_of(flag_name) {
		Some(FlagValue::String(s)) => Some(s),
		_ => None,
	}
}
