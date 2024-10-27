use clap::Parser;

mod cli;
mod wasm_component_loader;

fn main() {
    let cli = cli::Cli::parse();
    if let Err(err) = wasm_component_loader::load(&cli) {
        eprintln!("{err:?}");
    }
}
