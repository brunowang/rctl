use crate::opts::FileFormat;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

pub fn process_csv(input: &str, output: &str, format: FileFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // let mut player = HashMap::new();
        // for (i, value) in record.iter().enumerate() {
        //     let name = &headers[i];
        //     let value = value.to_string();
        //     player.insert(name, value);
        // }
        // ret.push(serde_json::json!(player));
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        FileFormat::Json => serde_json::to_string_pretty(&ret)?,
        FileFormat::Yaml => serde_yaml::to_string(&ret)?,
        FileFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
