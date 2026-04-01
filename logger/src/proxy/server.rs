use std::net::TcpListener;

use crate::api::router::Router;
use crate::api::route::{root_handler, hello_handler, about_handler};
use crate::models::request_log::Method;
use crate::proxy::handler;

pub fn start(port: &str) {
    let mut router = Router::new();

    router.add_route(Method::GET, "/", root_handler);
    router.add_route(Method::GET, "/hello", hello_handler);
    router.add_route(Method::GET, "/about", about_handler);

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to bind to port");

    println!("🚀 DevTrace Engine running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handler::handle_connection(stream, &router),
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}