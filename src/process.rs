use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut players = Vec::with_capacity(128);
    for res in reader.deserialize() {
        let player: Player = res?;
        players.push(player);
    }
    let json = serde_json::to_string_pretty(&players)?;
    fs::write(output, json)?;
    Ok(())
}
