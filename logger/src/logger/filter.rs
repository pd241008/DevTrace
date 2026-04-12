use std::collections::HashMap;
use crate::models::request::Method;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortBy {
    Duration,
}

#[derive(Debug, Clone)]
pub struct LogFilter {
    pub method: Option<Method>,
    pub status: Option<u16>,
    pub sort: Option<SortBy>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}



pub fn build_filter(query: &HashMap<String, String>) -> Result<LogFilter, String> {

    let method = match query.get("method") {
        Some(m) => match m.to_uppercase().as_str() {
            "GET" => Ok(Some(Method::GET)),
            "POST" => Ok(Some(Method::POST)),
            "PUT" => Ok(Some(Method::PUT)),
            "DELETE" => Ok(Some(Method::DELETE)),
            "" => Err("Method cannot be empty".to_string()), // Catch empty ?method=
            invalid => Err(format!("Invalid method: '{}'. Allowed: GET, POST, PUT, DELETE", invalid)),
        },
        None => Ok(None),
    }?;

    let status = match query.get("status") {
        Some(s) => match s.parse::<u16>() {
            Ok(val) => Ok(Some(val)),
            Err(_) => Err(format!("Invalid status: '{}'. Must be a number.", s)),
        },
        None => Ok(None),
    }?; 

   
    let limit = query.get("limit").and_then(|s| s.parse::<usize>().ok());
    let offset = query.get("offset").and_then(|s| s.parse::<usize>().ok());
    let sort = query.get("sort").cloned();

   
    Ok(LogFilter {
        method,
        status,
        sort,
        limit,
        offset,
    })
}