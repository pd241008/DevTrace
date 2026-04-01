mod api;
mod models;
mod proxy;

fn main() {
  
    proxy::server::start("8080");
}