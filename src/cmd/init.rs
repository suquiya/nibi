use combu::{
	Command, Context, Flag, action::bundle::Bundle, action_result, checks, done, flags, license,
	vector,
};
use combu::{FlagType, FlagValue, Vector, no_flag, yes_flag};
use std::path::{Path, PathBuf};
use std::{fs, io::ErrorKind::AlreadyExists};

use crate::app::config::default_config_file_type;
use crate::app::path::{file_name, get_abs_path_from_option};
use crate::cli::prompt::inquiry_str;
use crate::cmd::common::get_yes_no;
use crate::{
	app::config::{self, Config},
	cmd::common::overwrite_confirm,
};

use super::common::{get_flagged_yes_no, sub_help, take_to_string_option};

pub fn cmd() -> Command {
	Command::with_all_field(
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
	)
}

pub fn flags() -> Vector<Flag> {
	vector![
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
			vector![],
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
			vector!['s'],
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
	]
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
			dir_path: get_abs_path_from_option(bundle.args().front()),
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

impl InitConfig {
	pub fn should_prompt(&self) -> bool {
		!self.skip_prompt
			|| self.project_name.is_none()
			|| self.site_name.is_none()
			|| self.config_file_type.is_none()
			|| self.ingots_dir_path.is_none()
	}

	pub fn get_force_yes_no(&self) -> Option<bool> {
		if self.force { Some(true) } else { self.yes_no }
	}
}

fn init(mut init_config: InitConfig) {
	let dir_path = PathBuf::from(&init_config.dir_path);

	if init_config.should_prompt() {
		prompt_init_config(&mut init_config)
	}

	// init先フォルダの状態確認となければ作成
	if !create_root_dir(&dir_path, init_config.get_force_yes_no()) {
		print_early_exit_message();
		return;
	}

	let config = get_config_from_init_config(&init_config);
	if !create_config_file(&dir_path, &config, init_config.get_force_yes_no()) {
		print_early_exit_message();
		return;
	}

	if !create_src_dirs(&config, &dir_path) {
		print_early_exit_message();
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
			false
		}
	}
}

fn prompt_init_config(init_config: &mut InitConfig) {
	// init_configをプロンプトで補足する
	println!("input your project information for initialization.");
	if init_config.project_name.is_none() {
		let dir_path = file_name(&init_config.dir_path);
		let project_name = inquiry_str("project name", &dir_path);
		init_config.project_name = Some(project_name);
	};

	if init_config.site_name.is_none() {
		let pn = init_config.project_name.as_ref().unwrap();
		let sn = inquiry_str("site name", pn);
		init_config.site_name = Some(sn);
	}

	if init_config.config_file_type.is_none() {
		let config_file_type = inquiry_str("config file type", &default_config_file_type());
		init_config.config_file_type = Some(config_file_type);
	}
}

fn get_config_from_init_config(init_config: &InitConfig) -> Config {
	let project_name = match &init_config.project_name {
		Some(pn) => pn.clone(),
		None => file_name(&init_config.dir_path),
	};
	let site_name = match &init_config.site_name {
		Some(sn) => sn.clone(),
		None => project_name.clone(),
	};
	Config::new(project_name, site_name)
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
			let mut r: bool = true;
			for e in errs {
				if e.0.kind() == AlreadyExists {
					println!("ディレクトリ {} は既に存在します", e.1.display());
				} else {
					println!(
						"ディレクトリ {} の作成中にエラーが発生しました: {}",
						e.1.display(),
						e.0
					);
					r = false;
				}
			}
			r
		}
		_ => true,
	}
}

fn print_early_exit_message() {
	println!("初期化処理を中断し、プログラムを終了します。");
}
