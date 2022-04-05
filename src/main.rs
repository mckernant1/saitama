use crate::args::args::{Cli, Commands};
use clap::Parser;
use commands::punch;
use log::debug;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

mod args;
mod commands;
mod model;
mod orchestrator;
mod output;
mod util;
mod worker;

fn main() {
    let args: Cli = Cli::parse();
    let command = args.command;
    let log_level = args.verbose.log_level_filter();
    CombinedLogger::init(vec![TermLogger::new(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .expect("Logger failed to instantiate");

    debug!("Got Command {:?}", command);

    match command {
        Commands::Punch(p) => {
            punch::punch(p);
        }
    };
}
