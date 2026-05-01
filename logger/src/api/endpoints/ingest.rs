use crate::logger::model::RequestLog;
use crate::models::request::Request;
use crate::models::response::Response;
use crate::logger::store::LogStore;
use std::sync::Arc;

pub async fn ingest_log(req: &Request, store: Arc<LogStore>) -> Response {
    // 1. Check if the body exists (which we populated in proxy/handler.rs)
    if let Some(body_string) = &req.body {
        
        // 2. Parse the body directly into your RequestLog struct
        match serde_json::from_str::<RequestLog>(body_string) {
            Ok(log) => {
                // 3. Drop it onto the conveyor belt!
                store.add(log);
                
                return Response {
                    status: 202,
                    body: "{\"status\": \"Accepted\"}".to_string(),
                };
            }
            Err(e) => {
                return Response {
                    status: 400,
                    body: format!("{{\"error\": \"Invalid JSON: {}\"}}", e),
                };
            }
        }
    }

    Response {
        status: 400,
        body: "{\"error\": \"Missing body\"}".to_string(),
    }
}
