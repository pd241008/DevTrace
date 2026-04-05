use crate::logger::store::LogStore;
use crate::api::serializer;
use std::sync::Arc;

pub fn get_all_logs(store: Arc<LogStore>) -> String {
    let logs = store.get_all();
    serializer::to_json(logs)
}

pub fn get_latest_logs(store: Arc<LogStore>) -> String {
    let logs = store.get_all();

    match logs.last() {
        Some(log) => serializer::single_to_json(log.clone()),
        None => "{}".to_string(),
    }
}