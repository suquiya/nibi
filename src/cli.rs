use crate::cmd;
use combu::action_result;

pub mod prompt;
mod stdio;

/// Execute Program
pub fn run() -> action_result!() {
	return cmd::treed_cmd().run_with_auto_arg_collect();
}
