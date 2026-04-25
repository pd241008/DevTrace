use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone, Serialize)]
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
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => return None,
        };

        let path = parts.next()?.to_string();
        let _version = parts.next()?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        Some(Self {
            method,
            path,
            headers: HashMap::new(),
            body: None,
            timestamp,
        })
    }
}