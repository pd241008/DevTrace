use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::sync::mpsc;

pub struct LogStoreSQL {
    
    tx: mpsc::Sender<RequestLog>,
    

    pool: SqlitePool,
}

pub fn LogStoreSQL(path: &str, store: Arc<Mutex<LogStore>>) -> Response {


    
}
