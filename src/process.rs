use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            // TOML 需要一个结构化的数据格式，我们将使用一个包含数组的结构
            #[derive(Serialize)]
            struct TomlData {
                players: Vec<Value>,
            }
            let toml_data = TomlData { players: ret };
            toml::to_string_pretty(&toml_data)?
        },
    };

    fs::write(output, content)?;
    Ok(())
}
