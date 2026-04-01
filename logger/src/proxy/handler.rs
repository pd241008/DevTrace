use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::api::router::Router;
use crate::models::request_log::Request;

pub fn handle_connection(mut stream: TcpStream, router: &Router) {
    let buf_reader = BufReader::new(&mut stream);
    
    // Grab just the first line (e.g., "GET /hello HTTP/1.1")
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return, // Connection dropped or empty
    };

    println!("📥 Incoming: {}", request_line);

    // Step 1: Parse
    if let Some(req) = Request::parse(&request_line) {
        // Step 4: Route & Match
        let response = router.handle_request(req);
        
        // Write back to stream
        stream.write_all(response.to_http_string().as_bytes()).unwrap();
    } else {
        // Handle malformed requests
        let bad_req = "HTTP/1.1 400 Bad Request\r\n\r\nBad Request";
        stream.write_all(bad_req.as_bytes()).unwrap();
    }
}