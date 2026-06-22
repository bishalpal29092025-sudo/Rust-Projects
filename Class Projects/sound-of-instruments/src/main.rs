mod sound {
    pub mod instrument{
        pub fn clarinet() {
            println!("Clarinet is being played.");
        }
    }
}

// Bringing 'Clarinet' into scope using 'use'
use sound::instrument::clarinet;
fn main() {
    clarinet(); // Here we can Call clarinet directly 
}
