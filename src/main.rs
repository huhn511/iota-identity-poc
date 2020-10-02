
mod server;

use server::start_server;

fn main() {
    match start_server() {
        Ok(_) => {

        },
        Err(error) => {
            println!("Error: {}", error)
        }
    } 
}
