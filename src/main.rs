pub mod lib;

fn main() {
    let server_result = lib::server();
    match server_result {
        Ok(()) => println!("Server closed normally."),
        Err(err) => println!("Error closing server: {}", err)
    }
}