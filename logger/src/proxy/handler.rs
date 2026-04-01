use std::io::{BufRead, BufReader, Write, Read};
use std::net::TcpStream;
use std::time::Instant;
use std::collections::HashMap;

use crate::api::router::Router;
use crate::models::request_log::Request;
use crate::logger::logger::log;

pub fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut buf_reader = BufReader::new(&mut stream);

    // ✅ READ REQUEST LINE (NO .lines())
    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line).is_err() {
        return;
    }

    // ✅ HEADERS
    let mut headers = HashMap::new();

    loop {
        let mut line = String::new();

        if buf_reader.read_line(&mut line).is_err() {
            return;
        }

        let line = line.trim().to_string();

        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // ✅ PARSE REQUEST
    if let Some(mut req) = Request::parse(&request_line) {
        req.headers = headers;

        // ✅ BODY
        if let Some(content_length) = req.headers.get("Content-Length") {
            if let Ok(len) = content_length.parse::<usize>() {
                let mut body_buf = vec![0; len];

                if buf_reader.read_exact(&mut body_buf).is_ok() {
                    req.body = Some(String::from_utf8_lossy(&body_buf).to_string());
                }
            }
        }

        let start_time = Instant::now();

        let response = router.handle_request(&req);

        let latency = start_time.elapsed();

        log(&req, &response, latency);

        let _ = stream.write_all(response.to_http_string().as_bytes());

    } else {
        let bad_req = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\nBad Request";
        let _ = stream.write_all(bad_req.as_bytes());
    }
}