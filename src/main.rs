// rctl csv -i input.csv output.json --header -d ','

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rctl", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to other formats.")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(long, default_value_t = true)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn verify_file(filepath: &str) -> Result<String, &'static str> {
    if Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        Err("Input file does not exist")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    date_of_birth: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut players = Vec::with_capacity(128);
            for res in reader.deserialize() {
                let player: Player = res?;
                players.push(player);
            }
            let json = serde_json::to_string_pretty(&players)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}
