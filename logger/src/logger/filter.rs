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
}