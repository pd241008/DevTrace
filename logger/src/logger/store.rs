use std::sync::{Arc, Mutex};

use super::model::RequestLog;

#[derive(Clone)]
pub struct LogStore {
    logs: Arc<Mutex<Vec<RequestLog>>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, log: RequestLog) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(log);
    }

    pub fn get_all(&self) -> Vec<RequestLog> {
        let logs = self.logs.lock().unwrap();
        logs.clone()
    }

    pub fn get_one(&self, index: usize) -> Option<RequestLog> {
        let logs = self.logs.lock().unwrap();
        logs.get(index).cloned()
    }
}