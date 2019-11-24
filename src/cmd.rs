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
        ("init", Some(_)) => {}
        ("new", Some(_)) => {}
        (_, _) => {}
    }
}
