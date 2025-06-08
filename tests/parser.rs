use dairyshop::parser::parse_herd_xml;
use std::io::Write;
use std::process::Command;

#[test]
fn test_parse_herd_xml() {
    let xml = "<herd><labyak name=\"Yakky\" age=\"5\" sex=\"f\" /></herd>";
    let mut file = tempfile::NamedTempFile::new().expect("temp file");
    file.write_all(xml.as_bytes()).unwrap();
    let path = file.path().to_str().unwrap();
    let herd = parse_herd_xml(path).expect("parse xml");
    assert_eq!(herd.yaks.len(), 1);
    let yak = &herd.yaks[0];
    assert_eq!(yak.name, "Yakky");
    assert_eq!(yak.initial_age_years, 5.0);
    assert_eq!(yak.sex, 'f');
}

#[test]
fn test_parse_herd_xml_negative_age() {
    let xml = "<herd><labyak name=\"Yakky\" age=\"-1\" sex=\"f\" /></herd>";
    let mut file = tempfile::NamedTempFile::new().expect("temp file");
    file.write_all(xml.as_bytes()).unwrap();
    let path = file.path().to_str().unwrap();
    assert!(parse_herd_xml(path).is_err());
}

#[test]
fn test_parse_herd_xml_invalid_sex() {
    let xml = "<herd><labyak name=\"Yakky\" age=\"5\" sex=\"x\" /></herd>";
    let mut file = tempfile::NamedTempFile::new().expect("temp file");
    file.write_all(xml.as_bytes()).unwrap();
    let path = file.path().to_str().unwrap();
    assert!(parse_herd_xml(path).is_err());
}

#[test]
fn test_parse_herd_xml_duplicate_names() {
    let xml = "<herd><labyak name=\"Yakky\" age=\"5\" sex=\"f\" /><labyak name=\"Yakky\" age=\"6\" sex=\"f\" /></herd>";
    let mut file = tempfile::NamedTempFile::new().expect("temp file");
    file.write_all(xml.as_bytes()).unwrap();
    let path = file.path().to_str().unwrap();
    assert!(parse_herd_xml(path).is_err());
}

#[test]
fn test_parse_herd_xml_missing_file() {
    assert!(parse_herd_xml("missing_file.xml").is_err());
}

#[test]
fn test_cli_non_numeric_days_elapsed() {
    let bin = env!("CARGO_BIN_EXE_dairyshop");
    let path = format!("{}/tests/herd.xml", env!("CARGO_MANIFEST_DIR"));
    let output = Command::new(bin)
        .arg(&path)
        .arg("abc")
        .output()
        .expect("run binary");
    assert!(!output.status.success());
}

#[test]
fn test_cli_negative_days_elapsed() {
    let bin = env!("CARGO_BIN_EXE_dairyshop");
    let path = format!("{}/tests/herd.xml", env!("CARGO_MANIFEST_DIR"));
    let output = Command::new(bin)
        .arg(&path)
        .arg("-3")
        .output()
        .expect("run binary");
    assert!(!output.status.success());
}

#[test]
fn test_cli_positive_days_elapsed() {
    let bin = env!("CARGO_BIN_EXE_dairyshop");
    let path = format!("{}/tests/herd.xml", env!("CARGO_MANIFEST_DIR"));
    let output = Command::new(bin)
        .arg(&path)
        .arg("13")
        .output()
        .expect("run binary");
    assert!(output.status.success());
}
