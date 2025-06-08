use dairyshop::parser::parse_herd_xml;
use std::io::Write;

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
