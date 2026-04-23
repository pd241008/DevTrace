use std::time::{SystemTime, UNIX_EPOCH};

use crate::logger::model::RequestLog;
use crate::models::request::Request;
use crate::models::response::Response;

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros()) 
        .unwrap_or(0)
}

pub fn build_log(
    request: Request,
    response: Response,
    start_time: u128,
) -> RequestLog {
    let end_time = now();

    RequestLog {
        request,
        response,
        start_time,
        end_time,
        duration_ms: end_time - start_time,
    }
}