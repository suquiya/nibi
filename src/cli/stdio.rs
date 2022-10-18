use std::io;
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
