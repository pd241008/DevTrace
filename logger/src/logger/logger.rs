use crate::models::request_log::Request;
use crate::models::response_log::Response;
use std::time::Duration;

pub fn log(req: &Request, res: &Response, latency: Duration) {
    println!(
        "[{}] {:?} {} -> {} ({} ms)",
        req.timestamp,
        req.method,
        req.path,
        res.status,
        latency.as_millis()
    );
}