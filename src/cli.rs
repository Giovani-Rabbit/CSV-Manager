use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    pub path_file: String,

    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
pub enum Action {
    Filter {
        condition: String,
        #[arg(short = 'l', long)]
        limit: Option<usize>,
    },
    Show {
        #[arg(short = 'l', long)] // -l or --limit
        limit: Option<usize>,
    },
    Export,
}
