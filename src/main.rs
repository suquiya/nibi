//BSD 3-Clause License
//
//Copyright (c) 2019, suquiya
//All rights reserved.
//
//please read LICENSE and README.md
use nibi::cli_parser as cli;
use nibi::cmd;

fn main() {
    cmd::exec_from_cli(cli::nibi_basic());
}
