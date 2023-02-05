use entrypoint::Application;

// Application struct
pub struct App {
    pub name: String,
    pub version: String,
}

impl Application for App {}

/// main creates and runs an Io application
fn main() {
    // define and run the application
    App {
        name: "Io74XX".to_string(),
        version: "0.0.1".to_string(),
    }.run();
}

