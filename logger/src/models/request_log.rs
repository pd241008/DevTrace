#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

impl Request {
    // Parses the first line of the TCP stream
    pub fn parse(request_line: &str) -> Option<Self> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 { return None; }

        let method = match parts[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => return None,
        };

        Some(Request {
            method,
            path: parts[1].to_string(),
        })
    }
}