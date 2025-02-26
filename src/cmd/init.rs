use combu::{
	action::bundle::Bundle, action_result, checks, done, flags, license, vector, Command, Context,
	Flag,
};
use combu::{flag, no_flag, yes_flag, FlagType, FlagValue, Vector};
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::{fs, io::ErrorKind::AlreadyExists};

use crate::cmd::common::get_yes_no;
use crate::{
	app::config::{self, Config},
	cmd::common::overwrite_confirm,
};

use super::common::{get_flagged_yes_no, sub_help, take_to_string_option};

pub fn cmd() -> Command {
	return Command::with_all_field(
		"init".to_owned(),
		Some(route),
		String::default(),
		String::default(),
		license![],
		None,
		"nibi init [directory path: default is current]".to_owned(),
		flags![yes, no, ["import-ingots-dir-path"=>[>string,="import ingots path"]]],
		flags![],
		vector![],
		String::default(),
		vector![sub_help()],
	);
}

pub fn flags() -> Vector<Flag> {
	return vector![
		yes_flag!(),
		no_flag!(),
		Flag::with_all_field(
			"import-ingots-dir-path".to_owned(),
			"import ingots path: 取り込みたいingotsを収納したディレクトリのパス".to_owned(),
			vector!['i'],
			Vector::default(),
			FlagType::String,
			FlagValue::from(""),
		),
		Flag::with_all_field(
			"skip-create-prompt".to_owned(),
			"skip prompt for project information: 初期化時のプロンプトをスキップ".to_owned(),
			vector!['s'],
			Vector::default(),
			FlagType::Bool,
			FlagValue::Bool(false),
		),
		Flag::with_all_field(
			"force".to_owned(),
			"force init dir: ファイルが存在しても強制的に初期化".to_owned(),
			vector!['f'],
			Vector::default(),
			FlagType::Bool,
			FlagValue::Bool(false),
		),
		Flag::with_all_field(
			"project-name".to_owned(),
			"project name: プロジェクト名".to_owned(),
			vector!['p'],
			vector![=>String, "pn", "p-name", "name"],
			FlagType::String,
			FlagValue::from("")
		),
		Flag::with_all_field(
			"site-name".to_owned(),
			"site name: サイト名".to_owned(),
			vector!['n'],
			vector![=>String,"sn","s-name"],
			FlagType::String,
			FlagValue::from("")
		),
		Flag::with_all_field(
			"config-file-type".to_owned(),
			"file type of config: コンフィグファイルの形式".to_owned(),
			vector!['t'],
			vector![=>String, "cft", "config-ft"],
			FlagType::String,
			FlagValue::from("ron")
		)
	];
}

pub fn route(cmd: Command, ctx: Context) -> action_result!() {
	checks!(cmd, ctx, [error, help, version, license]);
	init_action(cmd, ctx)
}

pub fn init_action(cmd: Command, ctx: Context) -> action_result!() {
	let bundle = Bundle::new(ctx, cmd);

	init(InitConfig::from(bundle));
	done!()
}

struct InitConfig {
	pub dir_path: PathBuf,
	pub yes_no: Option<bool>,
	pub ingots_dir_path: Option<String>,
	pub project_name: Option<String>,
	pub site_name: Option<String>,
	pub skip_prompt: bool,
	pub config_file_type: Option<String>,
	pub force: bool,
}

impl From<Bundle> for InitConfig {
	fn from(mut bundle: Bundle) -> Self {
		InitConfig {
			dir_path: match bundle.args().front() {
				Some(path) => {
					let path = PathBuf::from(path);
					match &path {
						p if p.is_absolute() => path,
						_ => {
							let p = current_dir().unwrap().join(path);
							p.canonicalize().unwrap()
						}
					}
				}
				None => current_dir().unwrap(),
			},
			yes_no: get_flagged_yes_no(&bundle),
			ingots_dir_path: take_to_string_option(&mut bundle, "import-ingots-dir-path"),
			project_name: take_to_string_option(&mut bundle, "project-name"),
			site_name: take_to_string_option(&mut bundle, "site-name"),
			skip_prompt: bundle.is_flag_true("skip_prompt"),
			config_file_type: take_to_string_option(&mut bundle, "config-file-type"),
			force: bundle.is_flag_true("force"),
		}
	}
}

fn init(init_config: InitConfig) {
	let dir_path = PathBuf::from(&init_config.dir_path);
	let yes_no = init_config.yes_no.clone();

	// init先フォルダの状態確認となければ作成
	if !create_root_dir(&dir_path, init_config.yes_no) {
		print_early_exit_message();
		return;
	}

	let config = get_config(&dir_path, &init_config);
	if !create_config_file(&dir_path, &config, yes_no) {
		print_early_exit_message();
		return;
	}

	if !create_src_dirs(&config, &dir_path) {
		print_early_exit_message();
		return;
	}
}

fn create_root_dir(dir_path: &Path, yes_no: Option<bool>) -> bool {
	match fs::create_dir(dir_path) {
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
							// フォルダが空の場合
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
			} else {
				// 他のエラーなら再作成してみてエラーハンドリング
				match fs::create_dir_all(dir_path) {
					Ok(_) => {
						println!(
							"プロジェクトフォルダ: {}を作成しました。",
							dir_path.display()
						);
						return true;
					}
					Err(e) => err = e,
				}
			}
			println!("指定されたパスでエラーが発生しました:{}", err);
			return false;
		}
	}
}

fn get_config(dir_path: &Path, init_config: &InitConfig) -> Config {
	if init_config.skip_prompt {
		// プロンプトのskipリクエストがはいっているならフラグと引数の結果からコンフィグを返す
		return;
	};

	// コンフィグ作成に必要な情報を入力してもらう
}

fn get_config_from_init_config(init_config: &InitConfig) -> Config {}

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
							println!("コンフィグの初期化処理に失敗しました: {}", err);
							false
						}
						_ => true,
					},
					_ => {
						println!("上書きしません。");
						false
					}
				}
			} else {
				println!("コンフィグファイルの作成中にエラーが発生しました: {}", err);
				false
			}
		}
	}
}

fn create_src_dirs(config: &Config, root_dir: &Path) -> bool {
	match config.get_dir_conf().create_src_dirs(root_dir) {
		Err(errs) => {
			for e in errs {
				println!(
					"ディレクトリ {} の作成中にエラーが発生しました: {}",
					e.1.display(),
					e.0
				);
			}
			false
		}
		_ => true,
	}
}

fn print_early_exit_message() {
	println!("初期化処理を中断し、プログラムを終了します。");
}
