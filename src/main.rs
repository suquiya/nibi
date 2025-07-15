use combu::ActionResult;
//BSD 3-Clause License
//
//Copyright (c) 2019, suquiya
//All rights reserved.
//
//please read LICENSE and README.md
use nibi::cli;

fn main() {
	if let Ok(ActionResult::Result(cmd, ctx)) = cli::run() {
		if let Some(action) = cmd.action {
			let _ = action(cmd, ctx);
		}
	}
}
