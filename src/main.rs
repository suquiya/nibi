#![warn(missing_docs)]
//! Nibi is a static site generator. (WIP, implementing)
//!
//!BSD 3-Clause License
//!
//!Copyright (c) 2019, suquiya
//! All rights reserved.
//!
//!please read LICENSE and README.md
use combu::ActionResult;

use nibi::cli;

/// The entry point for the Nibi CLI.
pub fn main() {
	if let Ok(ActionResult::Result(cmd, ctx)) = cli::run()
		&& let Some(action) = cmd.action
	{
		let _ = action(cmd, ctx);
	}
}
