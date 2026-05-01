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
           
        }
        Commands::Serve => {
            println!("🧠 DevTrace Engine — Server mode");
            
       
            let store = Arc::new(LogStore::new().await);
            println!("🚀 Infrastructure ready. Starting proxy server...\n");
            server::start("8080", store).await;
        }
    }
}
