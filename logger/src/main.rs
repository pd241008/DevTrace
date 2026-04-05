mod api;
mod models;
mod proxy;
mod logger;

use crate::proxy::server;
use crate::logger::store::LogStore;
use std::sync::Arc;

fn main() {
    let store = Arc::new(LogStore::new());

    server::start("8080", store);
}