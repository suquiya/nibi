use combu::{
	action::bundle::Bundle, action_result, checks, done, flags, license, vector, Command, Context,
	Flag,
};
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;

use crate::{app::config, cmd::common::overwrite_confirm};

use super::common::{get_flagged_yes_no, sub_help};

pub fn cmd() -> Command {
	return Command::with_all_field(
		"init".to_owned(),
		Some(route),
		String::default(),
		String::default(),
		license![],
		None,
		"nibi init [directory path: default is current]".to_owned(),
		flags![yes, no],
		flags![],
		vector![],
		String::default(),
		vector![sub_help()],
	);
}

fn route(cmd: Command, ctx: Context) -> action_result!() {
	checks!(cmd, ctx, [error, help, version, license]);
	init_action(cmd, ctx)
}
pub fn init_action(cmd: Command, ctx: Context) -> action_result!() {
	let bundle = Bundle::new(ctx, cmd);

	let dir_path = match bundle.args().front() {
		Some(path) => path,
		None => ".",
	};
	let yes_no = get_flagged_yes_no(&bundle);
	dir_init(dir_path, yes_no);
	done!()
}

fn dir_init(dir_path: &str, yes_no: Option<bool>) {
	let config_path = config::get_config_path(Path::new(dir_path), "ron");

	match config::create_config_file(&config_path) {
		Ok(_) => {
			println!("configファイルを作成しました: {}", config_path.display());
		}
		Err(err) => {
			println!("エラー: {}", err);
			if err.kind() == AlreadyExists {
				println!("configファイルが既に存在します");
				let yes_no = overwrite_confirm(yes_no);
				match yes_no {
					Some(true) => {
						match config::overwrite_config_file(&config_path) {
							Ok(_) => {
								println!("configファイルを上書きしました");
							}
							Err(err) => {
								println!(
									"エラーが発生しました: {}\r\n処理を中断し、プログラムを終了します。",
									err
								);
							}
						};
					}
					Some(false) => {
						println!("上書きしません。init処理を中断します。");
					}
					None => println!("上書きしません。init処理を中断して終了します。"),
				}
			}
		}
	}
}
