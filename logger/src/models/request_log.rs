use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timestamp: u128,
}

impl Request {
    pub fn parse(request_line: &str) -> Option<Self> {
        let mut parts = request_line.split_whitespace();

        let method = match parts.next()? {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => return None,
        };

        let path = parts.next()?.to_string();
        let _version = parts.next()?; // ensure HTTP version

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        Some(Request {
            method,
            path,
            headers: HashMap::new(),
            body: None,
            timestamp,
        })
    }
}