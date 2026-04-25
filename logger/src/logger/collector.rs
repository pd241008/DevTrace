use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Utc, TimeZone};

use crate::logger::model::RequestLog;
use crate::models::request::Request;
use crate::models::response::Response;

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros()) 
        .unwrap_or(0)
}

/// Converts microseconds to a human-readable UTC string.
pub fn format_timestamp(micros: u128) -> String {
    let secs = (micros / 1_000_000) as i64;
    let nsecs = ((micros % 1_000_000) * 1_000) as u32;
    Utc.timestamp_opt(secs, nsecs)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Invalid Timestamp".to_string())
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
        timestamp_human: format_timestamp(start_time),
    }
}