mod csv;
mod genpass;

use crate::cmd::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;
use std::path::Path;

pub use csv::FileFormat;

#[derive(Debug, Parser)]
#[command(name = "rctl", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

fn verify_file(filepath: &str) -> Result<String, &'static str> {
    if Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        Err("Input file does not exist")
    }
}
