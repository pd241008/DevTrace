use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use tokio::sync::mpsc;

/// LogStore — The Conveyor Belt
///
/// No longer holds data in memory. Instead, it holds:
///   - A `Sender` half of a Tokio MPSC channel (the conveyor belt input)
///   - A `SqlitePool` for reading logs via SQL queries
///
/// Writes are non-blocking: the proxy tosses a log onto the belt via `try_send`.
/// A background worker (spawned at startup) pulls logs off the belt and INSERTs them.
/// Reads go directly to SQLite with dynamic SQL queries.
pub struct LogStore {
    tx: mpsc::Sender<RequestLog>,
    pool: SqlitePool,
}

impl LogStore {
    /// Boot Sequence: connect to SQLite, ensure schema, build the belt, spawn the worker.
    pub async fn new() -> Self {
        // ── Step 1a: Connect to the Database ──
        let db_dir = "database";
        let db_path = "database/devtrace.db";

        if let Err(e) = std::fs::create_dir_all(db_dir) {
            eprintln!("⚠️  Could not create database directory: {}", e);
        }

        let db_url = format!("sqlite://{}?mode=rwc", db_path);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("❌ Failed to connect to SQLite at database/devtrace.db");

        println!("✅ SQLite connection pool established.");

        // ── Step 1b: Ensure the Schema ──
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                method TEXT NOT NULL,
                path TEXT NOT NULL DEFAULT '',
                status INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                start_time INTEGER NOT NULL,
                end_time INTEGER NOT NULL
            );"
        )
        .execute(&pool)
        .await
        .expect("❌ Failed to create logs table");

        println!("✅ Schema verified — logs table ready.");

        // ── Step 1c: Build the Conveyor Belt ──
        let (tx, mut rx) = mpsc::channel::<RequestLog>(10_000);

        // ── Step 2: Spawn the Background Worker ──
        let worker_pool = pool.clone();
        tokio::spawn(async move {
            println!("👷 Background Event Worker started. Waiting for logs...");

            while let Some(log) = rx.recv().await {
                let method_str = format!("{:?}", log.request.method);

                let result = sqlx::query(
                    "INSERT INTO logs (method, path, status, duration_ms, start_time, end_time)
                     VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&method_str)
                .bind(&log.request.path)
                .bind(log.response.status as i64)
                .bind(log.duration_ms as i64)
                .bind(log.start_time as i64)
                .bind(log.end_time as i64)
                .execute(&worker_pool)
                .await;

                match result {
                    Ok(_) => {
                        // Silently succeed — the belt keeps moving
                    }
                    Err(e) => {
                        eprintln!("❌ Database Write Error: {}", e);
                    }
                }
            }

            println!("⚠️  Background worker shutting down — channel closed.");
        });

        Self { tx, pool }
    }

    /// Step 3: The Fast Write — toss the log onto the conveyor belt.
    ///
    /// This is NON-BLOCKING. No mutex, no disk I/O.
    /// If the belt is full (10,000 logs backed up), the log is dropped with a warning.
    pub fn add(&self, log: RequestLog) {
        if let Err(e) = self.tx.try_send(log) {
            eprintln!("⚠️  Conveyor belt full! Dropping log: {}", e);
        }
    }

    /// Step 4: The Indexed Read — translates LogFilter into a dynamic SQL query.
    ///
    /// Called by the API when the dashboard requests filtered logs.
    /// Instead of iterating a Vec, this builds a SQL WHERE clause and lets
    /// SQLite's indexed engine do the heavy lifting.
    pub async fn get_filtered_logs(&self, filter: &LogFilter) -> Vec<RequestLog> {
        let mut query = String::from("SELECT method, path, status, duration_ms, start_time, end_time FROM logs WHERE 1=1");
        let mut bind_values_str: Vec<String> = Vec::new();
        let mut bind_values_int: Vec<i64> = Vec::new();
        let mut param_order: Vec<&str> = Vec::new(); // track order: "str" or "int"

        if let Some(ref method) = filter.method {
            query.push_str(" AND method = ?");
            bind_values_str.push(format!("{:?}", method));
            param_order.push("str");
        }

        if let Some(status) = filter.status {
            query.push_str(" AND status = ?");
            bind_values_int.push(status as i64);
            param_order.push("int");
        }

        // Sorting
        if let Some(sort_by) = &filter.sort {
            match sort_by {
                SortBy::Duration => {
                    query.push_str(" ORDER BY duration_ms DESC");
                }
            }
        } else {
            query.push_str(" ORDER BY id DESC");
        }

        // Pagination
        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        // Build and execute query with dynamic binds
        let mut q = sqlx::query(&query);
        let mut str_idx = 0;
        let mut int_idx = 0;
        for kind in &param_order {
            match *kind {
                "str" => {
                    q = q.bind(&bind_values_str[str_idx]);
                    str_idx += 1;
                }
                "int" => {
                    q = q.bind(bind_values_int[int_idx]);
                    int_idx += 1;
                }
                _ => {}
            }
        }

        let rows = match q.fetch_all(&self.pool).await {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("❌ Database Read Error: {}", e);
                return Vec::new();
            }
        };

        // Convert rows back into RequestLog structs
        rows.iter()
            .filter_map(|row| {
                let method_str: String = row.get("method");
                let path: String = row.get("path");
                let status: i64 = row.get("status");
                let duration_ms: i64 = row.get("duration_ms");
                let start_time: i64 = row.get("start_time");
                let end_time: i64 = row.get("end_time");

                let method = match method_str.as_str() {
                    "GET" => crate::models::request::Method::GET,
                    "POST" => crate::models::request::Method::POST,
                    "PUT" => crate::models::request::Method::PUT,
                    "DELETE" => crate::models::request::Method::DELETE,
                    _ => return None,
                };

                Some(RequestLog {
                    request: crate::models::request::Request {
                        method,
                        path,
                        headers: std::collections::HashMap::new(),
                        body: None,
                        timestamp: start_time as u128,
                    },
                    response: crate::models::response::Response {
                        status: status as u16,
                        body: String::new(),
                    },
                    start_time: start_time as u128,
                    end_time: end_time as u128,
                    duration_ms: duration_ms as u128,
                    timestamp_human: crate::logger::collector::format_timestamp(start_time as u128),
                })
            })
            .collect()
    }

    /// Fetch the single most recent log entry.
    pub async fn get_latest(&self) -> Option<RequestLog> {
        let filter = LogFilter {
            method: None,
            status: None,
            sort: None,
            limit: Some(1),
            offset: Some(0),
        };
        self.get_filtered_logs(&filter).await.into_iter().next()
    }
}