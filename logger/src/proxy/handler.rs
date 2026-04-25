use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::routes;
use crate::logger::store::LogStore;
use crate::logger::collector::{now, build_log};
use crate::models::request::Request;
use crate::models::response::Response;
use crate::api::router::Router;

pub async fn handle_connection(
    mut stream: TcpStream,
    router: &Router,
    store: Arc<LogStore>,
) {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);

    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line).await.is_err() {
        return;
    }

    let mut headers = HashMap::new();

    loop {
        let mut line = String::new();

        if buf_reader.read_line(&mut line).await.is_err() {
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

        // 🔥 API INTERCEPT — check if this is a /logs request
        let response = if let Some(api_res) =
            routes::handle_api(&req.path, store.clone()).await
        {
            Response {
                status: 200,
                body: api_res,
            }
        } else {
            router.handle_request(&req)
        };

        // Build the log entry
        let log = build_log(req.clone(), response.clone(), start_time);

        // 🚀 The Fast Write — toss the log onto the conveyor belt
        // No mutex lock, no disk I/O. This returns in nanoseconds.
        store.add(log);

        let _ = writer.write_all(response.to_http_string().as_bytes()).await;
    } else {
        let _ = writer.write_all(
            b"HTTP/1.1 400 Bad Request\r\n\r\nBad Request"
        ).await;
    }
}