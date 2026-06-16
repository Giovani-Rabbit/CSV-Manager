use clap::Parser;

use crate::{
    cli::{Action, Cli},
    csv::Csv,
};

mod cli;
mod csv;
mod json;
mod show;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_params = Cli::parse();
    let csv = Csv::new(&cli_params.path_file)?;

    match cli_params.action {
        Some(Action::Filter { condition, limit }) => {
            let filtered_csv = csv.filter(&condition)?;
            show::print_table(&csv.headers, &filtered_csv, limit);
        }
        Some(Action::Show { limit }) => {
            show::print_table(&csv.headers, &csv.lines, limit);
        }
        Some(Action::Export) => {
            println!("{}", csv.to_json());
        }
        None => {}
    }

    Ok(())
}
