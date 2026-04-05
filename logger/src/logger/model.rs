use crate::models::request::Request;
use crate::models::response::Response;

#[derive(Debug, Clone)]
pub struct RequestLog {
    pub request: Request,
    pub response: Response,
    pub start_time: u128,
    pub end_time: u128,
    pub duration_ms: u128,
}