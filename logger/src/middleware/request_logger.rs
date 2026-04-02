use crate::logger::collector::{now, build_log};
use crate::logger::store::LogStore;
use crate::models::request::Request;
use crate::models::response::Response;

pub fn log_request<F>(
    req: Request,
    store: &LogStore,
    handler: F,
) -> Response
where
    F: Fn(Request) -> Response,
{
    let start_time = now();

    // Process request
    let res = handler(req.clone());

    // Build log
    let log = build_log(req, res.clone(), start_time);

    // Store log
    store.add(log);

    res
}
