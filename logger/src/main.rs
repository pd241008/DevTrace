mod api;
mod models;
mod proxy;
mod logger;
mod utils;
mod replayengine;


use replayengine::commands::{Cli, Commands};
use clap::Parser;
use crate::proxy::server;
use crate::logger::store::LogStore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🧠 DevTrace Engine — Initializing...");
    
 
    let cli = Cli::parse();


    match cli.command {
        Commands::Replay { id } => {
            println!("🧠 DevTrace Engine — Replay mode for ID: {}", id);
           
            let numeric_id: u64 = id.parse().expect("ID must be a valid integer");

            // 1. Connect to SQLite
            let store = LogStore::new().await;
            
            // 2. Fetch the Log
            if let Some(log) = store.get_log_by_id(numeric_id).await {
                println!("📦 Found Log: [{:?}] {}", log.request.method, log.request.path);
                println!("🚀 Firing Replay...");

                // 3. Reconstruct and Fire the Request
                let client = reqwest::Client::new();
                
                // We replay the request directly against the DevTrace proxy port (8080)
                let url = format!("http://127.0.0.1:8080{}", log.request.path);

                let req_builder = match log.request.method {
                    crate::models::request::Method::GET => client.get(&url),
                    crate::models::request::Method::POST => client.post(&url),
                    crate::models::request::Method::PUT => client.put(&url),
                    crate::models::request::Method::DELETE => client.delete(&url),
                };

                match req_builder.send().await {
                    Ok(resp) => {
                        println!("✅ Replay Success! Server responded with Status: {}", resp.status());
                    }
                    Err(e) => {
                        println!("❌ Replay Failed: {}", e);
                    }
                }
            } else {
                println!("❌ Replay Failed: Log ID {} not found in database.", id);
            }
        }
        Commands::Serve => {
            println!("🧠 DevTrace Engine — Server mode");
            
       
            let store = Arc::new(LogStore::new().await);
            println!("🚀 Infrastructure ready. Starting proxy server...\n");
            server::start("8080", store).await;
        }
    }
}
