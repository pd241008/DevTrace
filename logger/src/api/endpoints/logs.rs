use crate::logger::store::LogStore;
use std::sync::Arc;
use crate::models::response::Response;
use crate::logger::filter::build_filter;
use crate::utils::query_parsers::parse_query;
use serde_json;

/// Handler for GET /logs
pub async fn handle_logs(path: &str, store: Arc<LogStore>) -> Response {
    let query_map = parse_query(path);

    let filter = match build_filter(&query_map) {
        Ok(valid_filter) => valid_filter,
        Err(error_msg) => {
            return Response {
                status: 400,
                body: format!("{{\"error\": \"{}\"}}", error_msg),
            };
        }
    };

    let logs = store.get_filtered_logs(&filter).await;
    let json = serde_json::to_string_pretty(&logs).unwrap_or_else(|_| "[]".to_string());

    Response {
        status: 200,
        body: json,
    }
}

/// Handler for GET /logs/latest
pub async fn get_latest_logs(store: Arc<LogStore>) -> Response {
    match store.get_latest().await {
        Some(log) => {
            let json = serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string());
            Response {
                status: 200,
                body: json,
            }
        }
        None => Response {
            status: 200,
            body: "{}".to_string(),
        },
    }
}
