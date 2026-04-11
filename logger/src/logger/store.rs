use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};

pub struct LogStore {
    logs: Vec<RequestLog>,
}

impl LogStore {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn add(&mut self, log: RequestLog) {
        self.logs.push(log);
    }

    pub fn get_all(&self) -> &Vec<RequestLog> {
        &self.logs
    }

    pub fn get_filtered_logs(&self, filter: &LogFilter) -> Vec<RequestLog> {
        let mut result: Vec<RequestLog> = self
            .logs
            .iter()
            .filter(|log| {
                let mut matches = true;

                if let Some(ref method) = filter.method {
                    matches = matches && &log.method == method;
                }

                if let Some(status) = filter.status {
                    matches = matches && log.status == status;
                }

                matches
            })
            .cloned()
            .collect();

        if let Some(sort_by) = &filter.sort {
            match sort_by {
                SortBy::Duration => {
                    result.sort_by(|a, b| b.duration.cmp(&a.duration));
                }
            }
        }

        result
    }
}