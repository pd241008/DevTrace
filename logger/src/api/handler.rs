use crate::logger::store::LogStore;
use crate::api::serializer;
use std::sync::{Arc, Mutex};
use crate::models::response::Response;
use crate::logger::filter::build_filter;
use crate::utils::query_parsers::parse_query; 

pub fn get_all_logs(store: Arc<Mutex<LogStore>>) -> String {
    if let Ok(locked_store) = store.lock() {
        let logs = locked_store.get_all();
        serializer::to_json(logs.clone())
    } else {
        "[]".to_string()
    }
}

pub fn get_latest_logs(store: Arc<Mutex<LogStore>>) -> String {
    if let Ok(locked_store) = store.lock() {
        let logs = locked_store.get_all();

        match logs.last() {
            Some(log) => serializer::single_to_json(log.clone()),
            None => "{}".to_string(),
        }
    } else {
        "{}".to_string()
    }
}





pub fn handle_logs(path: &str, store: Arc<Mutex<LogStore>>) -> Response {
    let query_map = parse_query(path);
    
    // 🔥 Check if the filter is valid before we ever touch the database
    let filter = match build_filter(&query_map) {
        Ok(valid_filter) => valid_filter,
        Err(error_msg) => {
            // 🚨 The user sent garbage data. Reject the request immediately.
            return Response {
                status: 400, // 400 Bad Request
                body: format!("{{\"error\": \"{}\"}}", error_msg),
            };
        }
    };

    // ... The rest of your code remains exactly the same! ...
    if let Ok(locked_store) = store.lock() {
        let logs = locked_store.get_filtered_logs(&filter);
        let json = serde_json::to_string(&logs).unwrap_or_else(|_| "[]".to_string());

        Response {
            status: 200,
            body: json,
        }
    } else {
        Response {
            status: 500,
            body: "{\"error\": \"Internal Server Error\"}".to_string(),
        }
    }
}