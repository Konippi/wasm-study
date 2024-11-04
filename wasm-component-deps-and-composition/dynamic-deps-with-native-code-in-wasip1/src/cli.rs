use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub wasm_file: String,
}
