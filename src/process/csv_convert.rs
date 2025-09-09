use crate::cmd::FileFormat;
use csv::Reader;
use serde::Serialize;
use serde_json::Value;
use std::fs;

#[derive(Serialize)]
struct DataWrap {
    data: Vec<Value>,
}

pub fn process_csv(input: &str, output: &str, format: FileFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = DataWrap {
        data: Vec::with_capacity(128),
    };
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // let mut entity = HashMap::new();
        // for (i, value) in record.iter().enumerate() {
        //     let name = &headers[i];
        //     let value = value.to_string();
        //     entity.insert(name, value);
        // }
        // ret.push(serde_json::json!(entity));
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.data.push(json_value);
    }

    let content = match format {
        FileFormat::Json => serde_json::to_string_pretty(&ret)?,
        FileFormat::Yaml => serde_yaml::to_string(&ret)?,
        FileFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
