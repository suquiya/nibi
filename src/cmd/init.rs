use combu::{
	action::bundle::Bundle, action_result, checks, done, flags, license, vector, Command, Context,
	Flag,
};
use std::path::Path;
use std::{fs, io::ErrorKind::AlreadyExists};

use crate::cmd::common::get_yes_no;
use crate::{
	app::config::{self, Config},
	cmd::common::overwrite_confirm,
};

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

pub fn route(cmd: Command, ctx: Context) -> action_result!() {
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
	init(dir_path, yes_no);
	done!()
}

fn init(dir_path: &str, yes_no: Option<bool>) {
	let dir_path = Path::new(dir_path);

	// init先フォルダの状態確認となければ作成
	if !create_root_dir(dir_path, yes_no) {
		println!("処理を中断し、プログラムを終了します");
		return;
	}
	// TODO: コンフィグ作成問答の実装
	let config = create_config();
	let success = create_config_file(dir_path, &config, yes_no);
	if !success {
		return;
	}
}

fn create_root_dir(dir_path: &Path, yes_no: Option<bool>) -> bool {
	match fs::create_dir_all(dir_path) {
		Ok(_) => {
			println!(
				"プロジェクトフォルダ: {}を作成しました。",
				dir_path.display()
			);
			true
		}
		Err(mut err) => {
			if err.kind() == AlreadyExists {
				// プロジェクトフォルダが存在する場合
				match dir_path.read_dir() {
					Ok(mut i) => {
						return if i.next().is_none() {
							true
						} else {
							println!("指定されたディレクトリは空ではありません");
							get_yes_no(
								yes_no,
								"このまま指定されたディレクトリを使用して初期化しますか？",
							)
							.unwrap_or(false)
						};
					}
					Err(e) => err = e,
				}
			}
			println!("指定されたパスでエラーが発生しました:{}", err);
			return false;
		}
	}
}

fn create_config() -> Config {
	Config::default()
}

fn create_config_file(dir_path: &Path, config: &Config, yes_no: Option<bool>) -> bool {
	let config_path = config::get_config_path(dir_path, "ron");
	match config::create_config_file(&config_path, config) {
		Ok(_) => {
			println!("configファイルを作成しました: {}", config_path.display());
			true
		}
		Err(err) => {
			if err.kind() == AlreadyExists {
				let yes_no = overwrite_confirm(yes_no);
				match yes_no {
					Some(true) => match config::reset_config_file(&config_path, config) {
						Err(err) => {
							println!(
									"コンフィグの初期化処理に失敗しました: {}\r\n処理を中断し、プログラムを終了します。",
									err
								);
							false
						}
						_ => true,
					},
					_ => {
						println!("上書きしません。init処理を中断してプログラムを終了します。");
						false
					}
				}
			} else {
				println!("コンフィグファイルの作成中にエラーが発生しました: {}\r\ninit処理を中断してプログラムを終了します。", err);
				false
			}
		}
	}
}
