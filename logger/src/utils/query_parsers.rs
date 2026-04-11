use std::collections::HashMap;

pub fn parse_query(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let query_string = match path.split_once('?') {
        Some((_, query)) => query,
        None => return map,
    };

    for pair in query_string.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if !key.is_empty() {
                map.insert(key.to_string(), value.to_string());
            }
        }
    }

    map
}