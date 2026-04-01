pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn to_http_string(&self) -> String {
        let status_text = match self.status {
            200 => "OK",
            404 => "Not Found",
            _ => "Unknown",
        };
        
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            status_text,
            self.body.len(),
            self.body
        )
    }
}