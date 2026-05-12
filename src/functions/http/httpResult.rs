use crate::context::{DiscordContext, FnOutput};

/// ZhttpResult{key;...}
/// Reads the response body of the last HTTP request and navigates into it using
/// the provided keys as a JSON path. Variadic — each arg is one level deeper.
///
/// Examples:
///   ZhttpResult{name}           → top-level "name" field
///   ZhttpResult{user;id}        → response.user.id
///   ZhttpResult{items;0;title}  → response.items[0].title  (0-based array index)
///   ZhttpResult{}               → returns the raw response body as-is
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let body = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { ctx.http_last_body.lock().await.clone() })
    });

    if body.is_empty() {
        return FnOutput::Text(String::new());
    }

    // No keys — return raw body
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::Text(body);
    }

    // Parse as JSON and navigate the key path
    let mut value: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return FnOutput::error("httpResult", "response is not valid JSON"),
    };

    for key in &keys {
        value = match &value {
            serde_json::Value::Object(map) => match map.get(key.as_str()) {
                Some(v) => v.clone(),
                None => return FnOutput::Text(String::new()),
            },
            serde_json::Value::Array(arr) => match key.parse::<usize>() {
                Ok(i) => match arr.get(i) {
                    Some(v) => v.clone(),
                    None => return FnOutput::Text(String::new()),
                },
                Err(_) => return FnOutput::error(
                    "httpResult",
                    format!("key '{}' is not a valid array index", key),
                ),
            },
            _ => return FnOutput::Text(String::new()),
        };
    }

    // Serialize the final value to a clean string
    let result = match &value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    };

    FnOutput::Text(result)
}
