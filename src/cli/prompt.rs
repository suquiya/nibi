use std::fmt::Display;

use cliclack::{ProgressBar, confirm, input, log, select, spinner};

/// Prompts the user with a yes/no question and returns the user's response.
pub fn yes_or_no(message: &str) -> Option<bool> {
	confirm(message).interact().ok()
}

/// Prompts the user with a yes/no question and returns the user's response, with a default value.
pub fn yes_or_no_with_default(message: &str, default: bool) -> bool {
	yes_or_no(message).unwrap_or(default)
}

/// Prompts the user with an input question and returns the user's response.
pub fn inquiry_str(message: &str, default: &str) -> String {
	input(message)
		.default_input(default)
		.interact()
		.ok()
		.unwrap_or(default.into())
}

/// Prompts the user with a selection question and returns the user's response.
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

/// A struct that wraps a `ProgressBar` and provides methods for starting and stopping a spinner.
pub struct Spinner {
	inner: ProgressBar,
}

impl Spinner {
	/// Starts the spinner with the given message.
	pub fn start(message: &str) -> Self {
		let spinner = spinner();
		spinner.start(message);
		Self { inner: spinner }
	}
	/// Stops the spinner with the given message.
	pub fn end(self, message: impl Display) {
		self.inner.stop(message);
	}
}

/// Shows an error message to the user.
pub fn show_error(message: impl Display) {
	let _ = log::error(message);
}
