mod api;
mod models;
mod proxy;
mod logger;

use crate::proxy::server;

fn main() {
    println!("🚀 DevTrace Engine running on http://127.0.0.1:8080");
    server::start("8080");
}