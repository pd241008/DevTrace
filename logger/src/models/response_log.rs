pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn to_http_string(&self) -> String {
        let status_text = match self.status {
            200 => "OK",
            404 => "Not Found",
            400 => "Bad Request",
            _ => "Unknown",
        };

        let body_bytes = self.body.as_bytes();

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\n{}",
            self.status,
            status_text,
            body_bytes.len(),
            self.body
        )
    }
}