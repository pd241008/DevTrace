mod api;
mod models;
mod proxy;
mod logger;
mod utils;

use crate::proxy::server;
use crate::logger::store::LogStore;
use std::sync::{Arc, Mutex};

fn main() {
    // Safely wrap the store for multi-threaded mutation
    let store = Arc::new(Mutex::new(LogStore::new()));
    
    server::start("8080", store);
}