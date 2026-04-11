use crate::cmd;
use combu::action_result;

/// The prompt module provides prompt-based user input functionality.
pub mod prompt;
/// The stdio module provides stdio-based user input functionality.
mod stdio;

/// Execute Program
pub fn run() -> action_result!() {
	cmd::treed_cmd().run_with_auto_arg_collect()
}
