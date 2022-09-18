use combu::{
    action_result, checks, copyright, crate_authors, crate_license, crate_version, done, license,
    vector, Command, Context,
};
use std::io;
use std::io::Write;

pub fn new() -> Command {
    return Command::with_all_field(
        "nibi".to_owned(),
        Some(nibi_root_action),
        crate_authors!().to_owned(),
        copyright!(2022, suquiya),
        license!(crate_license!().to_owned(),file_path=>"../LICENSE"),
        Some(crate_authors!().to_owned()),
        "nibi [subcommand] [options]".to_owned(),
        vector![],
        vector![],
        vector![],
        crate_version!().to_owned(),
        vector![],
    );
}

fn nibi_root_action(cmd: Command, ctx: Context) -> action_result!() {
    checks!(cmd, ctx, [error, help, version, license]);
    done!()
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
