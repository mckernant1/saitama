#![allow(dead_code)]
extern crate clap;

use crate::args::Cli;
use crate::clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::fs;

#[path = "src/args/args.rs"]
mod args;

fn main() {
    let mut command = Cli::command();
    fs::create_dir_all("completions").unwrap();
    generate_to(Shell::Zsh, &mut command, "saitama", "completions").unwrap();
    generate_to(Shell::Bash, &mut command, "saitama", "completions").unwrap();
}
