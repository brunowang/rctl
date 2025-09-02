use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    let header = reader.headers()?.clone();
    for res in reader.records() {
        let record = res?;
        let mut player = HashMap::new();
        for (i, value) in record.iter().enumerate() {
            let name = &header[i];
            let value = value.to_string();
            player.insert(name, value);
        }
        players.push(serde_json::json!(player));
    }
    let json = serde_json::to_string_pretty(&players)?;
    fs::write(output, json)?;
    Ok(())
}
