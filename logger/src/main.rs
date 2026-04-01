mod proxy;

use std::{env, net::TcpListener};
use dotenvy::dotenv;

fn main() {
    // Load .env file
    dotenv().ok();

    
    let logger_url =
        env::var("LOGGER_URL").expect("LOGGER_URL not set in .env");

    println!("🚀 Starting server on {}", logger_url);

    let listener = TcpListener::bind(&logger_url)
        .expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                proxy::handle_connection(stream);
            }
            Err(e) => {
                eprintln!("❌ Connection failed: {}", e);
            }
        }
    }
}