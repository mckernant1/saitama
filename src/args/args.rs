use std::str::FromStr;
use chrono::{Duration, Month};
use clap::{Args, Parser, Subcommand, ArgEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Load test against an endpoint
    Punch(Punch),
}

#[derive(Args, Debug, Clone)]
pub struct Punch {
    /// Target URL
    #[clap(short, long)]
    pub url: String,

    /// Headers on the request
    #[clap(short = 'H', long)]
    pub headers: Vec<String>,

    /// How many worker threads to start
    #[clap(short, long, default_value_t = 10)]
    pub thread_count: u16,

    /// How many RPS to drive. This can max out on certain devices
    #[clap(short, long)]
    pub rps: u64,

    /// How long to run the test. Used with duration_unit
    #[clap(short, long)]
    pub duration: u64,

    /// What unit to run with the test
    #[clap(short = 'n', long, arg_enum, default_value_t = ChronoUnit::Minute )]
    pub duration_unit: ChronoUnit

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum ChronoUnit {
    Second,
    Minute,
    Hour
}

