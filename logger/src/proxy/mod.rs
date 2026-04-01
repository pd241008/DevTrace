use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn handle_connection(stream: TcpStream) {
    let reader = BufReader::new(stream);
    let mut http_request: Vec<String> = Vec::new();

    // Iterate through the lines safely
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                // An empty line marks the end of the HTTP headers
                if line.is_empty() {
                    break; 
                }
                http_request.push(line);
            }
            Err(e) => {
                // Log the error instead of panicking, then abort reading
                eprintln!("⚠️ Failed to read line (possibly invalid UTF-8): {}", e);
                break; 
            }
        }
    }

    if !http_request.is_empty() {
        println!("📥 Incoming Request:\n{:#?}", http_request);
    } else {
        println!("📥 Received empty or invalid request.");
    }
}