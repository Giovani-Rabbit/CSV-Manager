use clap::Parser;

use crate::{cli::Cli, csv::Csv};

mod cli;
mod csv;

fn main() {
    let cli_params = Cli::parse();

    let csv =
        Csv::new(&cli_params.path_file).unwrap_or_else(|err| panic!("Error on read csv: {err}"));

    let response = csv.filter("Identifier>12").unwrap();
    println!("{:?}", response);
}
