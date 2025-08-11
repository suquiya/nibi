use cliclack::{log, spinner};
use combu::{
	Command, Context, Flag, action::bundle::Bundle, action_result, done, flags, license, vector,
};
use combu::{FlagType, FlagValue, Vector, no_flag, yes_flag};
use exrs::cmd::exes;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io::ErrorKind::AlreadyExists};
use strum::VariantNames;

use crate::app::config::default_config_file_type;
use crate::app::fs::path::{file_name, get_abs_path_from_option, get_dir_path_string};
use crate::app::serde::FileType;
use crate::cli::prompt::{inquiry_str, selector};
use crate::cmd::common::{get_yes_no, get_yes_no_with_default};
use crate::route_common;
use crate::{
	app::config::{self, Config},
	cmd::common::overwrite_confirm,
};

use super::common::{get_flagged_yes_no, sub_help, take_to_bool_option, take_to_string_option};

pub fn cmd() -> Command {
	Command::with_all_field(
		"init".to_owned(),
		Some(route_common!(init_action)),
		String::default(),
		String::default(),
		license![],
		Some("init nibi project".to_owned()),
		"nibi init [directory path: default is current]".to_owned(),
		flags(),
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
			vector![=>String, "skip_create_prompt", "skip-prompt", "skip_prompt"],
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
			vector![=>String, "pn", "p-name", "name", "project_name", "p_name"],
			FlagType::String,
			FlagValue::from("")
		),
		Flag::with_all_field(
			"site-name".to_owned(),
			"site name: サイト名".to_owned(),
			vector!['s'],
			vector![=>String,"sn","s-name", "s_name", "site_name"],
			FlagType::String,
			FlagValue::from("")
		),
		Flag::with_all_field(
			"config-file-type".to_owned(),
			"file type of config: コンフィグファイルの形式".to_owned(),
			vector!['t'],
			vector![=>String, "cft", "config-ft", "config_ft", "config_file_type"],
			FlagType::String,
			FlagValue::from("ron")
		),
		Flag::with_all_field(
			"vcs".to_owned(),
			"will under management of vcs: 初期化後vcs(git)の初期化処理を実行してvcsの管理下に置く"
				.to_owned(),
			vector!['v'],
			vector![=>String, "git"],
			FlagType::Bool,
			FlagValue::Bool(true)
		)
	]
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
	pub config_file_type: Option<FileType>,
	pub force: bool,
	pub vcs: Option<bool>,
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
			config_file_type: take_to_string_option(&mut bundle, "config-file-type")
				.and_then(|s| FileType::from_str(&s).ok()),
			force: bundle.is_flag_true("force"),
			vcs: take_to_bool_option(&mut bundle, "vcs"),
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

	pub fn vcs_init(&self) -> bool {
		self.vcs.unwrap_or(true)
	}
}
//const INIT_EARLY_EXIT_MESSAGE: &str = "初期化処理を中断し、プログラムを終了します。";

fn get_early_exit_message() -> String {
	"初期化処理を中断し、プログラムを終了します。".to_string()
}

fn init(mut init_config: InitConfig) {
	let dir_path = PathBuf::from(&init_config.dir_path);

	if init_config.should_prompt() {
		prompt_init_config(&mut init_config)
	}

	// init先フォルダの状態確認となければ作成
	let spin = spinner();
	spin.start("プロジェクトフォルダを作成中...");
	match create_root_dir(&dir_path, init_config.get_force_yes_no()) {
		Ok(msg) => spin.stop(msg),
		Err(msg) => {
			spin.stop(msg);
			let _ = log::error(get_early_exit_message());
			return;
		}
	}

	let config = get_config_from_init_config(&init_config);

	let spin = spinner();
	spin.start("コンフィグファイルを作成中...");
	match create_config_file(
		&dir_path,
		&config,
		init_config.config_file_type.unwrap(),
		init_config.get_force_yes_no(),
	) {
		Ok(msg) => spin.stop(msg),
		Err(msg) => {
			spin.stop(msg);
			let _ = log::error(get_early_exit_message());
			return;
		}
	};

	match create_src_dirs(&config, &dir_path) {
		Ok(msg) => spin.stop(msg),
		Err(msg) => {
			spin.stop(msg);
			let _ = log::error(get_early_exit_message());
			return;
		}
	}

	if init_config.vcs_init() {
		init_vcs(&dir_path);
	}
}

fn create_root_dir(dir_path: &Path, yes_no: Option<bool>) -> Result<String, String> {
	match fs::create_dir(dir_path) {
		Ok(_) => Ok(format!(
			"プロジェクトフォルダ: {}を作成しました。",
			dir_path.display()
		)),
		Err(mut err) => {
			if err.kind() == AlreadyExists {
				// プロジェクトフォルダが存在する場合
				match dir_path.read_dir() {
					Ok(mut i) => {
						return if i.next().is_none() {
							// フォルダが空の場合
							Ok(format!(
								"空のプロジェクトフォルダ{}が既に存在します",
								dir_path.display()
							))
						} else {
							match get_yes_no(
								yes_no,
								"指定されたディレクトリは空ではありません\nこのまま指定されたディレクトリを使用して初期化しますか？",
							) {
								Some(true) => Ok(format!(
									"空のプロジェクトフォルダ{}を使用して初期化します",
									dir_path.display()
								)),
								_ => Err(format!(
									"{}が既に存在するため、初期化プロセスを中断しました。",
									dir_path.display()
								)),
							}
						};
					}
					Err(e) => err = e,
				}
			} else {
				// 他のエラーなら再作成してみてエラーハンドリング
				match fs::create_dir_all(dir_path) {
					Ok(_) => {
						return Ok(format!(
							"プロジェクトフォルダ: {}を作成しました。",
							dir_path.display()
						));
					}
					Err(e) => err = e,
				}
			}
			Err(format!("指定されたパスでエラーが発生しました:{err}"))
		}
	}
}

fn prompt_init_config(init_config: &mut InitConfig) {
	// init_configをプロンプトで補足する
	println!(
		"input your project information for initialization.: 初期化のためのプロジェクト情報を入力してください。"
	);
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
		let config_file_type = FileType::from_str(&selector(
			"config file type",
			FileType::VARIANTS,
			&default_config_file_type().to_string(),
		))
		.unwrap_or(default_config_file_type());
		init_config.config_file_type = Some(config_file_type);
	}

	if init_config.vcs.is_none() {
		init_config.vcs = Some(get_yes_no_with_default(
			init_config.yes_no,
			"初期化プロセスの最後にgit init を行いますか？",
			true,
		));
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

fn create_config_file(
	dir_path: &Path,
	config: &Config,
	file_type: FileType,
	yes_no: Option<bool>,
) -> Result<String, String> {
	let config_path = config::get_config_path(dir_path, file_type.to_string().as_str());
	match config::create_config_file(&config_path, config, file_type) {
		Ok(_) => Ok(format!(
			"configファイルを作成しました: {}",
			config_path.display()
		)),
		Err(err) => {
			if err.kind() == AlreadyExists {
				let yes_no = overwrite_confirm(yes_no);
				match yes_no {
					Some(true) => match config::reset_config_file(&config_path, config) {
						Err(err) => Err(format!("コンフィグの初期化処理に失敗しました: {err}")),
						_ => Ok("コンフィグファイルを上書きしました".to_string()),
					},
					_ => Err("上書きしません。".to_owned()),
				}
			} else {
				Err(format!(
					"コンフィグファイルの作成中にエラーが発生しました: {err}"
				))
			}
		}
	}
}

fn create_src_dirs(config: &Config, root_dir: &Path) -> Result<String, String> {
	match config.get_dir_conf().create_src_dirs(root_dir) {
		Err(errs) => {
			let mut msg = String::new();
			let mut r: bool = true;
			for e in errs {
				if !msg.is_empty() {
					msg.push('\n');
				}
				if e.0.kind() == AlreadyExists {
					msg.push_str(&format!("ディレクトリ {} は既に存在します", e.1.display()));
				} else {
					msg.push_str(&format!(
						"ディレクトリ {} の作成中にエラーが発生しました: {}",
						e.1.display(),
						e.0
					));
					r = false;
				}
			}
			if r { Ok(msg) } else { Err(msg) }
		}
		_ => Ok("ソースフォルダ群を作成しました".to_string()),
	}
}

fn init_vcs(dir_path: &Path) {
	let dir_path_str = get_dir_path_string(dir_path);
	let cmd = vec!["git", "init", &dir_path_str];
	println!("init vcs by: {}", cmd.join(" "));
	let result = exes(cmd);
	println!("{result}");
}
