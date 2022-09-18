//BSD 3-Clause License
//
//Copyright (c) 2019, suquiya
//All rights reserved.
//
//please read LICENSE and README.md
use nibi::cli;

fn main() {
	let _: Result<combu::ActionResult, combu::ActionError> = cli::new().run_with_auto_arg_collect();
}
