mod api;
mod models;
mod proxy;
mod logger;
mod utils;

use crate::proxy::server;
use crate::logger::store::LogStore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🧠 DevTrace Engine — Initializing...");

    // Step 1: Boot Sequence — connect to DB, create schema, build conveyor belt
    let store = Arc::new(LogStore::new().await);

    println!("🚀 Infrastructure ready. Starting proxy server...\n");

    // Step 3: Start the proxy server (now async-aware)
    server::start("8080", store).await;
}