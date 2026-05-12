use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_at_path};

/// ZjsonJoinArray{key;...;separator}
/// Joins all elements of the array at the given key path into a string using separator.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonJoinArray", "at least one key and a separator are required");
    }

    let separator = args.last().unwrap().clone();
    let keys: Vec<String> = args[..args.len() - 1]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();

    if keys.is_empty() {
        return FnOutput::error("jsonJoinArray", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text(String::new()),
        Some(root) => match get_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                let parts: Vec<String> = arr
                    .iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Null => String::new(),
                        other => other.to_string(),
                    })
                    .collect();
                FnOutput::Text(parts.join(&separator))
            }
            Some(_) => FnOutput::error("jsonJoinArray", "target is not an array"),
            None => FnOutput::Text(String::new()),
        },
    })
}
