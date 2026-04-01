use std::collections::HashMap;
use crate::models::{request_log::{Method, Request}, response_log::Response}; 

// Define a type alias for cleaner code. A Handler is just a function taking a Request and returning a Response.
type Handler = fn(Request) -> Response;

pub struct Router {
    // Maps (GET, "/hello") -> hello_handler function
    routes: HashMap<(Method, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Handler) {
        self.routes.insert((method, path.to_string()), handler);
    }

    // Step 4: Match route + return response
    pub fn handle_request(&self, req: Request) -> Response {
        let key = (req.method.clone(), req.path.clone());
        
        match self.routes.get(&key) {
            Some(handler) => handler(req), // Route found, execute handler!
            None => self.default_gateway(req), // Route not found, hit the fallback
        }
    }

    // 🔥 The Default Gateway / 404 Fallback
    fn default_gateway(&self, req: Request) -> Response {
        Response {
            status: 404,
            body: format!("404 - The path '{}' does not exist on this server.", req.path),
        }
    }
}