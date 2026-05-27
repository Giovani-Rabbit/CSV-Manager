use clap::Parser;

use crate::{
    cli::{Action, Cli},
    csv::Csv,
};

mod cli;
mod csv;
mod show;

fn main() {
    let cli_params = Cli::parse();

    let csv =
        Csv::new(&cli_params.path_file).unwrap_or_else(|err| panic!("Error on read csv: {err}"));

    match cli_params.action {
        Some(Action::Filter { condition, limit }) => match csv.filter(&condition) {
            Ok(filtered_csv) => show::print_table(&csv.headers, &filtered_csv, limit),
            Err(e) => eprintln!("Err: {e}"),
        },
        Some(Action::Show { limit }) => {
            show::print_table(&csv.headers, &csv.lines, limit);
        }
        Some(Action::Export) => println!("Testando"),
        None => {}
    }
}
