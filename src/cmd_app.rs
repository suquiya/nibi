use clap::{Arg, SubCommand};

fn def_parser<'a, 'b>() -> clap::App<'a, 'b> {
    return app_from_crate!()
        .arg(
            Arg::with_name("buildTarget")
            .help("Target directories for compile with nibi. This argment is not given, target directory will be current directory.")
            .default_value(".")
        ).arg(
            Arg::with_name("config")
                .help("config file if want to specify")
                .short("c")
                .long("config")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create new nibi object.Out of nibi project, create project named <objectName>. In nibi project, create post named <objectname>.")
                .arg(Arg::with_name("objectName").help("object name"))
                .subcommand(SubCommand::with_name("post"))
                .subcommand(SubCommand::with_name("proj"))
                .subcommand(SubCommand::with_name("theme")),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("create new nibi object.Out of nibi project, create project named <objectName>. In nibi project, create post named <objectname>.")
                .arg(Arg::with_name("objectName").help("object name"))
                .subcommand(SubCommand::with_name("newpost"))
                .subcommand(SubCommand::with_name("newproj"))
                .subcommand(SubCommand::with_name("newtheme")),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("init target directory for nibi")
                .arg(Arg::with_name("initTarget").help("directory name for new project.").default_value("."))
        )
        .subcommand(
            SubCommand::with_name("list")
            .about("list what are specidied by option. without the specification, this list project list.")
            .arg(Arg::with_name("type").help("type that is specified."))
        );
}

pub fn execute() {
    let cli_parser = def_parser();
    let matches = cli_parser.get_matches();
    if let Some(t) = matches.value_of("buildTarget") {
        println!("target: {}", t);
    }

    let (sub, _) = matches.subcommand();
    println!("subcommand: {}", sub);
    match matches.subcommand() {
        ("init", Some(_)) => {}
        ("new", Some(_)) => {}
        (_, _) => {}
    }
}
