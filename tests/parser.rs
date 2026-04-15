use dairyshop::parser::parse_name;
use std::process::Command;

#[test]
fn test_parse_name_simple() {
    assert_eq!(parse_name("Alice"), "Alice");
}

#[test]
fn test_parse_name_trims_whitespace() {
    assert_eq!(parse_name("  Bob  "), "Bob");
}

#[test]
fn test_parse_name_empty() {
    assert_eq!(parse_name(""), "");
}

#[test]
fn test_cli_no_args() {
    let bin = env!("CARGO_BIN_EXE_dairyshop");
    let output = Command::new(bin).output().expect("run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello, World!");
}

#[test]
fn test_cli_with_name() {
    let bin = env!("CARGO_BIN_EXE_dairyshop");
    let output = Command::new(bin)
        .arg("Rust")
        .output()
        .expect("run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello, Rust!");
}
