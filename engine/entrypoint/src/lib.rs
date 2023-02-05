// entrypoint to the game engine

/// Application entrypoint
pub trait Application {

    // name and version with sane default implementations
    fn name(&self) -> &'static str { "Unnamed Application" }
    fn version(&self) -> &'static str { "0.0.0" }

    // just an infinite loop for now
    fn run(&self) {
        // Log that the Io engine is running
        println!("Io Engine running {} v{}", self.name(), self.version());
        loop {}
    }
}


