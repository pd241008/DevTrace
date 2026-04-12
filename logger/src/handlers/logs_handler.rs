use crate::logger::store::LogStore;
use crate::logger::filter::build_filter;
use crate::models::response::Response;
use crate::utils::query_parser::parse_query; 
use std::sync::{Arc, Mutex};

pub fn handle_logs(path: &str, store: Arc<Mutex<LogStore>>) -> Response {
  
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


pub fn get_latest_logs(store: Arc<Mutex<LogStore>>) -> Response {
    if let Ok(locked_store) = store.lock() {
        let logs = locked_store.get_all();


        let json = match logs.last() {
            Some(log) => serde_json::to_string(log).unwrap_or_else(|_| "{}".to_string()),
            None => "{}".to_string(),
        };

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
