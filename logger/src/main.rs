mod api;
mod models;
mod proxy;
mod logger;

use crate::proxy::server;
use crate::logger::store::LogStore;
use std::sync::Arc;

fn main() {
    let store = Arc::new(LogStore::new());
    if request.path.starts_with("/logs") {
    if request.path == "/logs/latest" {
    
    server::start("8080", store);
    } else {
        let response = handle_logs(&request.path, &store);
        stream.write_all(response.to_http_string().as_bytes()).unwrap();
    }

}

}