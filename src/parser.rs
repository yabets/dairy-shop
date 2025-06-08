use crate::models::{Herd, Yak};
use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct HerdXml {
    #[serde(rename = "labyak")]
    labyaks: Vec<Labyak>,
}

#[derive(Debug, Deserialize)]
struct Labyak {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@age")]
    age: f32,
    #[serde(rename = "@sex")]
    sex: char,
}

impl From<Labyak> for Yak {
    fn from(labyak: Labyak) -> Self {
        Yak {
            name: labyak.name,
            initial_age_years: labyak.age,
            sex: labyak.sex,
        }
    }
}

impl From<HerdXml> for Herd {
    fn from(herd_xml: HerdXml) -> Self {
        let yaks: Vec<Yak> = herd_xml.labyaks.into_iter().map(Yak::from).collect();
        Herd { yaks }
    }
}

pub fn parse_herd_xml(file_path: &str) -> Result<Herd, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)
        .map_err(|e| format!("failed to read {}: {}", file_path, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // trim text nodes to match expected deserialization format
    let result: HerdXml = from_str(&contents)
        .map_err(|e| format!("failed to parse {}: {}", file_path, e))?;

    // Validate all entries before conversion
    let mut names: HashSet<String> = HashSet::new();
    for yak in &result.labyaks {
        if yak.age < 0.0 {
            return Err(format!("negative age for yak {}", yak.name).into());
        }
        if yak.sex != 'm' && yak.sex != 'f' {
            return Err(format!("invalid sex '{}' for yak {}", yak.sex, yak.name).into());
        }
        if !names.insert(yak.name.clone()) {
            return Err(format!("duplicate yak name: {}", yak.name).into());
        }
    }

    let herd = Herd::from(result);

    Ok(herd)
}
