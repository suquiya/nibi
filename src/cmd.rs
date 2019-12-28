use super::app::config;

pub fn exec_from_cli<'a, 'b>(cli: clap::App<'a, 'b>) {
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
            ("proj", Some(args_matches)) => {
                if let Some(project_name) = args_matches.value_of("project_name") {
                    new_proj(project_name)
                }
            }
            ("theme", Some(_)) => {}
            (_, _) => {}
        },
        (_, _) => {}
    }
}

fn init(target: &str) {
    println!("{}", target);
    config::create_config_file(target);
}

fn new_proj(project_name: &str) {
    println!("{}", project_name);
}
