use clap::{Arg, SubCommand};
use std::io;
use std::io::Write;

pub fn nibi_basic_parser<'a, 'b>() -> clap::App<'a, 'b> {
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
                .arg(Arg::with_name("force").help("force init flag").short("f").long("force"))
        )
        .subcommand(
            SubCommand::with_name("list")
            .about("list what are specidied by option. without the specification, this list project list.")
            .arg(Arg::with_name("type").help("type that is specified."))
        );
}
/// show prompt for yes or no. refs: https://github.com/conradkleinespel/rprompt/blob/master/src/lib.rs
pub fn prompt_yes_or_no(message: &str, retry: i16) -> Option<bool> {
    let mut answer: String;
    let mut stdout = std::io::stdout();
    let yes = vec!["yes", "y"];
    let no = vec!["no", "n"];
    let mut retry_count = retry;
    if retry < 0 {
        retry_count = std::i16::MAX;
    }

    loop {
        write!(stdout, "{} [Yes/No]: ", message).unwrap();
        stdout.flush().unwrap();
        answer = read_stdin().to_ascii_lowercase();
        match answer.as_str() {
            str if yes.contains(&str) => {
                return Some(true);
            }
            str if no.contains(&str) => {
                return Some(false);
            }
            _ => {
                if retry_count > 0 {
                    println!("[Y]esか[N]oで入力してください...残り{}回", retry_count);
                    retry_count -= 1;
                } else {
                    return None;
                }
            }
        }
    }
}

/// get string from stdin. refs: https://github.com/conradkleinespel/rprompt/blob/master/src/lib.rs
pub fn read_stdin() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => {
            println!("入力の読み取りに失敗しました: {}", err);
            return String::new();
        }
    };
    if !input.ends_with('\n') {
        println!(
            "予期せぬ終了記号を検出しました: {}",
            io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected end of input")
        );
        return String::new();
    }

    input.pop();

    if input.ends_with('\r') {
        input.pop();
    }

    return input;
}
