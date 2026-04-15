/// A simple greeting model.
pub struct Greeting {
    name: String,
}

impl Greeting {
    /// Creates a new greeting for the given name.
    pub fn new(name: &str) -> Self {
        Greeting {
            name: name.to_string(),
        }
    }

    /// Returns the greeting message.
    pub fn message(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
