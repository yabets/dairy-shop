use dairyshop::models::Greeting;

#[test]
fn test_greeting_default() {
    let greeting = Greeting::new("World");
    assert_eq!(greeting.message(), "Hello, World!");
}

#[test]
fn test_greeting_custom_name() {
    let greeting = Greeting::new("Rust");
    assert_eq!(greeting.message(), "Hello, Rust!");
}

#[test]
fn test_greeting_empty_name() {
    let greeting = Greeting::new("");
    assert_eq!(greeting.message(), "Hello, !");
}
