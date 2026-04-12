use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};
use crate::

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
  
        let mut matching_refs: Vec<&RequestLog> = self
            .logs
            .iter()
            .filter(|log| {
                let mut matches = true;

                if let Some(ref method) = filter.method {
                    matches = matches && &log.request.method == method;
                }

                if let Some(status) = filter.status {
                    matches = matches && log.response.status == status;
                }

                matches
            })
            .collect(); 
 
        if let Some(sort_by) = &filter.sort {
            match sort_by {
                SortBy::Duration => {
                    matching_refs.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms)); 
                }
            }
        }

        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(50);
        
        matching_refs
            .into_iter() 
            .skip(offset) 
            .take(limit)  
            .cloned()     
            .collect()    
    }

}