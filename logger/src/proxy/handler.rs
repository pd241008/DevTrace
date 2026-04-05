use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::routes;
use crate::logger::store::LogStore;
use crate::logger::collector::{now, build_log};
use crate::models::request::Request;
use crate::models::response::Response;
use crate::api::router::Router;

pub fn handle_connection(
    mut stream: TcpStream,
    router: &Router,
    store: Arc<LogStore>,
) {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line).is_err() {
        return;
    }

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

        if let Some((k, v)) = line.split_once(": ") {
            headers.insert(k.to_string(), v.to_string());
        }
    }

    if let Some(mut req) = Request::parse(&request_line) {
        req.headers = headers;

        let start_time = now();

        // 🔥 API INTERCEPT
        let response = if let Some(api_res) =
            routes::handle_api(&req.path, store.clone())
        {
            Response {
                status: 200,
                body: api_res,
            }
        } else {
            router.handle_request(&req)
        };

        // 🔥 BUILD LOG
        let log = build_log(req.clone(), response.clone(), start_time);
        store.add(log);

        let _ = stream.write_all(response.to_http_string().as_bytes());
    } else {
        let _ = stream.write_all(
            b"HTTP/1.1 400 Bad Request\r\n\r\nBad Request"
        );
    }
}