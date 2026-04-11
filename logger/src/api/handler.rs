use crate::logger::store::LogStore;
use crate::api::serializer;
use std::sync::{Arc, Mutex};

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