use combu::{
	action_result, checks, command::presets::func::help_tablize_with_alias_dedup, copyright,
	crate_authors, crate_license, crate_version, done, flags, license, preset_help_command, vector,
	Command, Context, Flag,
};

pub mod init;

pub fn root() -> Command {
	Command::with_all_field(
		"nibi".to_owned(),
		Some(root_action),
		crate_authors!().to_owned(),
		copyright!(2022, suquiya),
		license!(crate_license!().to_owned(),file_path=>"../LICENSE"),
		Some(crate_authors!().to_owned()),
		"nibi [subcommand] [options]".to_owned(),
		flags!(),
		flags!(help, version, license, authors, copyright),
		vector![],
		crate_version!().to_owned(),
		vector![
			preset_help_command!(help_tablize_with_alias_dedup),
			init::cmd(),
		],
	)
}

fn root_action(cmd: Command, ctx: Context) -> action_result!() {
	checks!(cmd, ctx, [error, help, version, license]);
	done!()
}
