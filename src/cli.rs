use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    pub path_file: String,

    #[arg(long = "where")]
    pub filter: Option<String>,

    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
pub enum Action {
    Show,
    Export,
}
