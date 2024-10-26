use clap::Parser;

mod cli;
mod file_reader;


fn main() {
    let cli = cli::Cli::parse();
    if let Err(err) = file_reader::read(&cli) {
        eprintln!("{err:?}");
    }
}
