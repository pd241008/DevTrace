use tokio::net::TcpListener;
use crate::logger::store::LogStore;
use crate::models::request::Method;
use crate::proxy::handler;
use crate::api::router::Router;
use crate::api::routes::root_handler;
use std::sync::Arc;

pub async fn start(port: &str, store: Arc<LogStore>) {
    let mut router = Router::new();

    // The root handler stays in routes.rs as requested
    router.add_route(Method::GET, "/", root_handler);

    let router = Arc::new(router);

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind");

    println!("🚀 DevTrace Engine running on http://{}", address);

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let store = store.clone();
                let router = router.clone();
                tokio::spawn(async move {
                    handler::handle_connection(stream, &router, store).await;
                });
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}