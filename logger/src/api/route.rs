use crate::models::{request_log::Request, response_log::Response};

// 📍 The new Root / Health Check route
pub fn root_handler(_req: Request) -> Response {
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

    Response {
        status: 200,
        body,
    }
}

pub fn hello_handler(_req: Request) -> Response {
    Response { status: 200, body: "Hello from Rust 🚀".to_string() }
}

pub fn about_handler(_req: Request) -> Response {
    Response { status: 200, body: "DevTrace Engine 🔥".to_string() }
}