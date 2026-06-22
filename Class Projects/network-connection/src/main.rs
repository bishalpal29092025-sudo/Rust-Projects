// Declare a public Module
pub mod network {

    // Private function within the public module, used to established a connection
    fn connect() {
        println!("Connection established.");
    }

    // Public function that initiates a network
    pub fn initiate_connection() {
        connect(); // Calls the private function
        println!("Initiating Connection.");
    }
}
fn main() {
    // Initialize a network connection -- This is possible because the public function is inside the scope thats why private function can called inside the public function
    network::initiate_connection();
}