//BSD 3-Clause License
//
//Copyright (c) 2019, suquiya
//All rights reserved.
//
//please read LICENSE and README.md

#[macro_use]
extern crate clap;

mod cmd_app;

fn main() {
    cmd_app::execute();
}
