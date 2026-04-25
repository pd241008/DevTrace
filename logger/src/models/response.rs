use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn to_http_string(&self) -> String {
        let status_text = match self.status {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            _ => "Unknown",
        };

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\n{}",
            self.status,
            status_text,
            self.body.len(),
            self.body
        )
    }
}