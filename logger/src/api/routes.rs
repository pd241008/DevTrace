//logger\src\api\route.rs
use crate::models::{request::Request, response::Response};
use crate::api::endpoints::{logs, info};
use crate::logger::store::LogStore;
use std::sync::Arc;

/// The root handler remains in this file as the primary entry point documentation.
pub fn root_handler(_req: &Request) -> Response {
    let body = r#"{
    "engine": "🧠 DevTrace Observability Core",
    "mission": "High-performance API traffic interception and persistent event auditing.",
    "status": "Active & Healthy",
    "version": "0.1.0",
    "infrastructure": {
        "conveyor_belt": "Tokio MPSC Channel (10k buffer)",
        "storage": "SQLite Persistent Store",
        "pattern": "CQRS (Command Query Responsibility Segregation)"
    },
    "navigation": {
        "GET /": "You are here (System Overview)",
        "GET /hello": "Friendly connectivity test",
        "GET /about": "Detailed documentation and Query Engine guide",
        "GET /logs": "Retrieve audit logs (supports SQL-backed filtering)",
        "GET /logs/latest": "Peek at the most recent captured event"
    }
}"#.to_string();

    Response { status: 200, body }
}

/// API route dispatcher — routes incoming traffic to the divided endpoint modules.
pub async fn handle_api(path: &str, store: Arc<LogStore>) -> Option<String> {
    // Strip query string for route matching
    let base_path = path.split('?').next().unwrap_or(path);

    match base_path {
        "/logs" => {
            let response = logs::handle_logs(path, store).await;
            Some(response.body)
        }
        "/logs/latest" => {
            let response = logs::get_latest_logs(store).await;
            Some(response.body)
        }
        "/hello" => {
            // We can also route static-ish handlers through here if needed
            let response = info::hello_handler(&Request::parse("GET /hello HTTP/1.1").unwrap());
            Some(response.body)
        }
        "/about" => {
            let response = info::about_handler(&Request::parse("GET /about HTTP/1.1").unwrap());
            Some(response.body)
        }
        _ => None,
    }
}