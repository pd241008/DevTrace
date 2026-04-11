//logger\src\api\route.rs
use crate::models::{request::Request, response::Response};
use crate::api::handler;
use crate::logger::store::LogStore;
use std::sync::{Arc, Mutex};


pub fn root_handler(_req: &Request) -> Response {
    let body = r#"{
    "name": "DevTrace Engine",
    "status": "online",
    "version": "0.1.0",
    "available_routes": [
        "GET /",
        "GET /hello",
        "GET /about"
    ]
}"#.to_string();

    Response { status: 200, body }
}

pub fn hello_handler(_req: &Request) -> Response {
    Response {
        status: 200,
        body: "Hello from Rust 🚀".to_string(),
    }
}

pub fn about_handler(_req: &Request) -> Response {
    Response {
        status: 200,
        body: "DevTrace Engine 🔥".to_string(),
    }
}


pub fn handle_api(path: &str, store: Arc<Mutex<LogStore>>) -> Option<String> {
    match path {
        "/logs" => Some(handler::get_all_logs(store)),
        "/logs/latest" => Some(handler::get_latest_logs(store)),
        _ => None,
    }
}