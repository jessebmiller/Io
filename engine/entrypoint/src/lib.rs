// entrypoint to the game engine

/// Application entrypoint
pub trait Application {

    // just an infinite loop for now
    fn run(&self) {
        // Log that the Io engine is running
        println!("Io Engine running...");
        loop {}
    }
}


