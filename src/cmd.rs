use combu::{
	Command, Context, Flag, action_result, checks,
	command::presets::func::help_tablize_with_alias_dedup, copyright, crate_authors, crate_license,
	crate_version, done, flags, license, vector,
};
use common::sub_help;

use crate::nibi_copyright;

pub mod build;
mod common;
pub mod init;
pub mod new;

pub fn treed_cmd() -> Command {
	Command::with_all_field(
		"nibi".to_owned(),
		Some(root_action),
		crate_authors!().to_owned(),
		nibi_copyright!(),
		license!(crate_license!().to_owned(),file_path=>"../LICENSE"),
		Some(crate_authors!().to_owned()),
		"nibi [subcommand] [options]".to_owned(),
		flags!(),
		flags!(help, version, license, authors, copyright),
		vector![],
		crate_version!().to_owned(),
		vector![sub_help(), init::cmd(), build::cmd(), new::threed_cmd()],
	)
}

fn root_action(cmd: Command, ctx: Context) -> action_result!() {
	checks!(cmd, ctx, [error, help, version, license]);
	println!("サブコマンドの指定がないため、ヘルプを表示します");
	let help = help_tablize_with_alias_dedup(&cmd, &ctx);
	println!("{help}");
	done!()
}
