use crate::logger::model::RequestLog;


pub fn to_json(logs: Vec<RequestLog>) -> String {
    let mut result = String::from("[");

    for (i, log) in logs.iter().enumerate() {
        result.push_str(&single_to_json(log.clone()));

        if i != logs.len() - 1 {
            result.push(',');
        }
    }

    result.push(']');
    result
}

pub fn single_to_json(log: RequestLog) -> String {
    format!(
        r#"{{
            "method":"{:?}",
            "path":"{}",
            "status":{},
            "duration":{}
        }}"#,
        log.request.method,
        log.request.path,
        log.response.status,
        log.duration_ms
    )
}