use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub pattern: String,
    pub file_name: String,
}
