use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::sync::mpsc;
use crate::logger::model::RequestLog;
use crate::logger::filter::{LogFilter, SortBy};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::sync::mpsc;

pub struct LogStore {

    tx: mpsc::Sender<RequestLog>,
    
  
    pool: SqlitePool,
}

impl LogStore {
    // We make initialization async because connecting to a DB is an async operation
  
    pub async fn new() -> Self {
       
        let db_dir = "database";
        let db_path = "database/devtrace.db";

        if let Err(e) = std::fs::create_dir_all(db_dir) {
            eprintln!("Warning: Could not create database directory: {}", e);
        }


        let db_url = format!("sqlite://{}?mode=rwc", db_path);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect to SQLite at logger/database/devtrace.db");

     
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                method TEXT NOT NULL,
                status INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                start_time INTEGER NOT NULL
            );"
        )
        .execute(&pool)
        .await
        .expect("Failed to create logs table");

        let (tx, mut rx) = mpsc::channel::<RequestLog>(10000);

     
        let worker_pool = pool.clone();
        tokio::spawn(async move {
            println!("👷 Background Event Worker started...");
            while let Some(log) = rx.recv().await {
                let result = sqlx::query(
                    "INSERT INTO logs (method, status, duration_ms, start_time) 
                     VALUES (?, ?, ?, ?)"
                )
                .bind(log.request.method)
                .bind(log.response.status)
                .bind(log.duration_ms)
                .bind(log.start_time as i64) 
                .execute(&worker_pool)
                .await;

                if let Err(e) = result {
                    eprintln!("❌ Database Write Error: {}", e);
                }
            }
        });

        Self { tx, pool }
    }
    
   
    pub fn add(&self, log: RequestLog) {
      
        if let Err(e) = self.tx.try_send(log) {
            eprintln!("Event Bus overloaded, dropping log: {}", e);
        }
    }
}


   

  

