use demand::{Confirm, DemandOption, Input, Select, Spinner};

pub fn yes_or_no(message: &str) -> Option<bool> {
	Confirm::new(message).run().ok()
}

pub fn yes_or_no_with_default(message: &str, default: bool) -> bool {
	yes_or_no(message).unwrap_or(default)
}

pub fn inquiry_str(message: &str, default: &str) -> String {
	let title = message.to_owned() + ": ";
	Input::new(title)
		.inline(true)
		.placeholder(default)
		.default_value(default)
		.validation(|s| {
			if s.is_empty() {
				Ok(())
			} else {
				Err("空文字は入力できません")
			}
		})
		.run()
		.ok()
		.unwrap_or(default.into())
}

pub fn selector(message: &str, options: &[&str], default: &str) -> String {
	let mut s = Select::new(message);
	for option in options {
		let demand_option = if *option == default {
			DemandOption::new(*option).selected(true)
		} else {
			DemandOption::new(*option)
		};
		s = s.option(demand_option);
	}

	s.run().ok().unwrap_or(default).into()
}

pub fn run_with_spinner<'scope, 'spinner: 'scope, F>(
	message: &str,
	func: F,
) -> Result<String, String>
where
	F: FnOnce(&mut SpinnerActionRunner<'spinner>) -> String + Send + 'scope,
{
	let r = Spinner::new(message).run(func);
}
