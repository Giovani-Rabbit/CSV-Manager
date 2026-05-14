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

    if let Some(condition) = cli_params.filter {
        match csv.filter(&condition) {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("Erro: {e}"),
        };
    }

    match cli_params.action {
        Some(Action::Show) => {
            show::show_table(&csv.headers, &csv.lines, 2);
        }
        Some(Action::Export) => println!("Testando"),
        None => {}
    }
}
