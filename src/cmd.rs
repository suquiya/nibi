/*pub fn exec_from_cli<'a, 'b>(cli: clap::App<'a, 'b>) {
    let matches = cli.get_matches();
    if let Some(t) = matches.value_of("buildTarget") {
        println!("target: {}", t);
    }

    let (sub, _) = matches.subcommand();
    if sub == "" {
        println!("no subcommand");
    } else {
        println!("subcommand: {}", sub);
    }

    match matches.subcommand() {
        ("init", Some(sub_matches)) => {
            if let Some(init_target_dir) = sub_matches.value_of("initTarget") {
                init(init_target_dir);
            }
        }
        ("new", Some(sub_matches)) => match sub_matches.subcommand() {
            ("post", Some(_)) => {}
            ("workbench", Some(args_matches)) => {
                if let Some(project_name) = args_matches.value_of("project_name") {
                    new_proj(project_name)
                }
            }
            ("theme", Some(_)) => {}
            (_, _) => {}
        },
        ("test", Some(sub_matches)) => {
            println!("{:?}", sub_matches);
        }
        (_, _) => {}
    }
}

fn init(target_dir_path: &str) {
    println!("{}", target_dir_path);
    let config_path = config::get_config_path(Path::new(target_dir_path), "ron");
    match config::create_config_file(&config_path) {
        Ok(_) => {
            println!("{} をworkbenchとして初期化しました。", target_dir_path);
        }
        Err(err) => {
            println!("エラー: {}", err);
            if err.kind() == AlreadyExists {
                println!("configファイルが既に存在します");
                match cli::prompt_yes_or_no("上書きしますか？", -1) {
                    Some(true) => {
                        match config::overwrite_config_file(&config_path) {
                            Ok(_) => {
                                println!("configファイルを上書きしました");
                                println!(
                                    "{} をworkbenchフォルダとして初期化しました",
                                    target_dir_path
                                )
                            }
                            Err(err) => {
                                println!(
                                    "エラーが発生しました: {}\r\n処理を中断し、プログラムを終了します。",err
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
    };
}

fn new_proj(project_name: &str) {
    println!("{}", project_name);
}*/
