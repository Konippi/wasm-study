use std::{fs::File, io::{BufRead, BufReader}};

use crate::cli::Cli;

pub fn read(cli: &Cli) -> anyhow::Result<()> {
    let file = File::open(&cli.file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{line}");
        }
    }
    Ok(())
}