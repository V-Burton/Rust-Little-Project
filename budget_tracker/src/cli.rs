use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action{
    Add,
    Display,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Budget Tracker",
    about = "A command line budget tracker written in Rust"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}