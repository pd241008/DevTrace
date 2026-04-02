mod api;
mod models;
mod proxy;
mod logger;

use crate::proxy::server;

fn main() {
    
    server::start("8080");
}
