use cliclack::{confirm, input, select};

pub fn yes_or_no(message: &str) -> Option<bool> {
	confirm(message).interact().ok()
}

pub fn yes_or_no_with_default(message: &str, default: bool) -> bool {
	yes_or_no(message).unwrap_or(default)
}

pub fn inquiry_str(message: &str, default: &str) -> String {
	input(message)
		.default_input(default)
		.interact()
		.ok()
		.unwrap_or(default.into())
}

pub fn selector(message: &str, options: &[&str], default: &str) -> String {
	let mut s = select(message);
	for option in options {
		let opt_val = *option;
		s = s.item(
			opt_val,
			opt_val,
			if opt_val == default { "default" } else { "" },
		);
	}

	s.interact().ok().unwrap_or(default).into()
}
