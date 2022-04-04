#![allow(dead_code)]
extern crate clap;

use std::fs;
use clap_complete::{Shell, generate_to};
use crate::clap::CommandFactory;
use crate::args::Cli;


#[path = "src/args/args.rs"]
mod args;

fn main() {
    let mut command = Cli::command();
    fs::create_dir_all("completions").unwrap();
    generate_to(Shell::Zsh, &mut command, "saitama", "completions").unwrap();
    generate_to(Shell::Bash, &mut command, "saitama", "completions").unwrap();
}