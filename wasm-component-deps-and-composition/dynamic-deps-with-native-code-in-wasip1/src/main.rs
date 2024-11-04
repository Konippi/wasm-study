use clap::Parser;

mod cli;
mod say;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(e) = say::execute(&cli) {
        eprintln!("{e}");
    }
}
