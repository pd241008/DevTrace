use crate::logger::store::LogStore;
use crate::logger::model::RequestLog;
use crate::models::request::Method;
use crate::models::response::Response;
use crate::utils::query_parser::parse_query;

#[derive(Debug)]
pub struct LogFilter {
    pub method: Option<Method>,
    pub status: Option<u16>,
    pub sort: Option<String>,
}

pub fn handle_logs(path: &str, store: &LogStore) -> Response {
    let query_map = parse_query(path);
    let filter = build_filter(&query_map);

    let logs = store.get_filtered_logs(&filter);

    let json = logs_to_json(&logs);

    Response {
        status: 200,
        body: json,
    }
}

fn build_filter(query: &std::collections::HashMap<String, String>) -> LogFilter {
    let method = query.get("method").and_then(|m| match m.as_str() {
        "GET" => Some(Method::GET),
        "POST" => Some(Method::POST),
        _ => None,
    });

    let status = query
        .get("status")
        .and_then(|s| s.parse::<u16>().ok());

    let sort = query.get("sort").cloned();

    LogFilter {
        method,
        status,
        sort,
    }
}

fn logs_to_json(logs: &Vec<RequestLog>) -> String {
    let mut out = String::from("[");

    for (i, log) in logs.iter().enumerate() {
        let entry = format!(
            "{{\"method\":\"{:?}\",\"path\":\"{}\",\"status\":{},\"duration\":{}}}",
            log.method,
            log.path,
            log.status,
            log.duration
        );

        out.push_str(&entry);

        if i != logs.len() - 1 {
            out.push(',');
        }
    }

    out.push(']');
    out
}