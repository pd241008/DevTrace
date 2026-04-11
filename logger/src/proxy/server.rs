use std::net::TcpListener;
use crate::logger::store::LogStore;
use crate::models::request::Method;
use crate::proxy::handler;
use crate::api::router::Router;
use crate::api::routes::{root_handler, hello_handler, about_handler};
use std::sync::{Arc, Mutex};

pub fn start(port: &str, store: Arc<Mutex<LogStore>>) {
    let mut router = Router::new();

    router.add_route(Method::GET, "/", root_handler);
    router.add_route(Method::GET, "/hello", hello_handler);
    router.add_route(Method::GET, "/about", about_handler);

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to bind");

    println!("🚀 DevTrace Engine running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handler::handle_connection(stream, &router, store.clone());
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}