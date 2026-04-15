pub mod models;
pub mod parser;

/// Returns the default greeting message.
pub fn hello() -> String {
    models::Greeting::new("World").message()
}
