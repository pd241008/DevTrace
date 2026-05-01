use crate::models::request::Request;
use crate::models::response::Response;

/// Handler for GET /hello
pub fn hello_handler(_req: &Request) -> Response {
    let body = r#"{
    "message": "Hello from the DevTrace Rust Proxy! 🦀",
    "status": "Success",
    "telemetry_note": "This request was just captured with microsecond precision and tossed onto the conveyor belt for background persistence.",
    "next_step": "Visit /logs to see this request appear in the persistent audit trail."
}"#.to_string();

    Response { status: 200, body }
}

/// Handler for GET /about
pub fn about_handler(_req: &Request) -> Response {
    let body = r#"{
    "about": "DevTrace is designed for zero-latency observability.",
    "how_it_works": {
        "1. Interception": "Internal proxy requests are captured automatically.",
        "2. External Agents": "External apps can push telemetry data to the Universal Gate (POST /api/ingest).",
        "3. The Conveyor Belt": "All events (internal and external) are dropped onto a Tokio MPSC Channel.",
        "4. Persistence": "A background worker saves logs to SQLite without blocking the main request flow.",
        "5. Querying": "The Query Engine translates URL parameters into optimized SQL queries."
    },
    "ingestion_api_guide": {
        "endpoint": "POST /api/ingest",
        "payload": "Requires a JSON body matching the internal RequestLog struct (request, response, duration_ms)",
        "behavior": "Fire-and-forget. Logs are asynchronously queued.",
        "response": "202 Accepted if queued successfully, 400 Bad Request on invalid JSON"
    },
    "query_engine_guide": {
        "description": "Filter and sort logs directly at the database level using query parameters.",
        "parameters": {
            "method": "Filter by HTTP method (e.g., GET, POST)",
            "status": "Filter by HTTP status code (e.g., 200, 500)",
            "limit": "Max number of logs to return (default: 50)",
            "offset": "Skip N logs for pagination",
            "sort": "Set to 'duration' to find the slowest requests"
        },
        "examples": [
            "/logs?status=500",
            "/logs?method=POST&limit=10",
            "/logs?sort=duration&limit=5"
        ]
    }
}"#.to_string();

    Response { status: 200, body }
}
