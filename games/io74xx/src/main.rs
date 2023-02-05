use entrypoint::Application;

// Application struct
pub struct App {
    pub name: &'static str,
    pub version: &'static str,
}

impl Application for App {
    fn name(&self) -> &'static str { self.name }
    fn version(&self) -> &'static str { self.version }
}

/// main creates and runs an Io application
fn main() {
    // define and run the application
    App {
        name: "Io74XX",
        version: "0.0.1",
    }.run();
}

