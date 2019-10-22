//BSD 3-Clause License
//
//Copyright (c) 2019, suquiya
//All rights reserved.
//
//please read LICENSE and README.md

//extern crate nibi;

#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};

fn main() {
    let app = app_from_crate!()
        .arg(Arg::with_name("target")
            .help("Target directories for compile with nibi. This argment is not given, target directory will be current directory.")
        )
        .arg(Arg::with_name("config")
            .help("config file if want to specify")
            .short("c")
            .long("config")
            .takes_value(true))
        .subcommand(SubCommand::with_name("new")
            .about("create new nibi project")
            .arg(Arg::with_name("projectName")
            .help("directory name for new project.")));
    let matches = app.get_matches();
    if let Some(target) = matches.value_of("target") {
        println!("target: {}", target);
    }
}
