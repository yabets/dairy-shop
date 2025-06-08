use crate::models::{Herd, Yak};
use quick_xml::Reader;
use quick_xml::de::from_str;
use serde::Deserialize;
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
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    dbg!(&contents);

    let mut reader = Reader::from_str(&contents);
    reader.config_mut().trim_text(true);

    let result: HerdXml = from_str(&contents).unwrap();

    let herd = Herd::from(result);

    Ok(herd)
}
