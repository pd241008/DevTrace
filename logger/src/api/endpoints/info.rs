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
        "1. Ingestion": "Requests are captured and sent to a background worker via an MPSC 'Conveyor Belt'.",
        "2. Persistence": "The worker saves logs to a SQLite database without blocking the main request flow.",
        "3. Querying": "The Query Engine translates your URL parameters into optimized SQL queries."
    },
    "query_engine_guide": {
        "description": "Filter and sort logs directly at the database level using query parameters.",
        "parameters": {
            "method": "Filter by HTTP method (e.g., GET, POST, PUT, DELETE)",
            "status": "Filter by HTTP status code (e.g., 200, 404, 500)",
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
