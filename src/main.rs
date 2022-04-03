use crate::args::{Cli, Commands, Punch};
use clap::Parser;
use log::{debug};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

mod args;
mod punch;
mod util;

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
