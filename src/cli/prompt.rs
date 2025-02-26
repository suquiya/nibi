use promkit::preset::readline::Readline;

pub fn yes_or_no(message: &str) -> Option<bool> {
	let confirm = Readline::default()
		.prefix(format!("{} [Yes/No]: ", message))
		.validator(
			|text: &str| -> bool {
				["yes", "y", "no", "n", "Y", "N", "YES", "NO", "Yes", "No"]
					.iter()
					.any(|yn| *yn == text)
			},
			|_| String::from("Accepts only 'y' or 'n' as an answer"),
		)
		.prompt();
	match confirm {
		Ok(mut prompt) => {
			let r = prompt.run();
			match r {
				Ok(prompt) => {
					let text = prompt.to_lowercase();
					if text.starts_with('y') {
						Some(true)
					} else if text.starts_with('n') {
						Some(false)
					} else {
						None
					}
				}
				Err(_) => return None,
			}
		}
		Err(_) => None,
	}
}
