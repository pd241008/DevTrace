use std::collections::HashMap;
use crate::models::{
    request::{Method, Request},
    response::Response,
};

// Handler now takes reference
type Handler = fn(&Request) -> Response;

pub struct Router {
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

    pub fn handle_request(&self, req: &Request) -> Response {
        let key = (req.method.clone(), req.path.clone());

        match self.routes.get(&key) {
            Some(handler) => handler(req),
            None => self.default_gateway(req),
        }
    }

    fn default_gateway(&self, req: &Request) -> Response {
        Response {
            status: 404,
            body: format!("404 - The path '{}' does not exist.", req.path),
        }
    }
}