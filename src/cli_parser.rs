use clap::{Arg, SubCommand};

pub fn nibi_basic<'a, 'b>() -> clap::App<'a, 'b> {
    let arg_config = Arg::with_name("config")
        .help("config file if want to specify")
        .short("c")
        .long("config")
        .takes_value(true);
    return app_from_crate!()
        .arg(
            Arg::with_name("buildTarget")
            .help("Target directories for compile with nibi. This argment is not given, target directory will be current directory.")
            .default_value(".")
        ).arg(
            arg_config.clone()
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create a new nibi object. Out of nibi workbench, create  a new workbench named <objectName>.")
                .subcommand(SubCommand::with_name("post"))
                .subcommand(SubCommand::with_name("workbench")
                    .arg(Arg::with_name("workbhenchName").help("workbench name"))
                )
                .subcommand(SubCommand::with_name("theme")),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("create a new nibi object. Out of nibi workbrnch, create a new workbench named <objectName>")    
                .arg(Arg::with_name("objectName").help("object name"))
                .subcommand(SubCommand::with_name("newpost"))
                .subcommand(SubCommand::with_name("newworkbench"))
                .subcommand(SubCommand::with_name("newtheme")),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("init target directory for nibi")
                .arg(Arg::with_name("initTarget").help("directory name for new project.").default_value("."))
                .arg(arg_config)
        )
        .subcommand(
            SubCommand::with_name("list")
            .about("list what are specidied by option. without the specification, this list project list.")
            .arg(Arg::with_name("type").help("type that is specified."))
        );
}
