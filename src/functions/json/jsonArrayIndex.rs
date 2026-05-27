use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_at_path, infer_value};

/// ZjsonArrayIndex{key;...;value}
/// Returns the 0-based index of the first occurrence of value in the array.
/// Returns -1 if not found.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonArrayIndex", crate::error_messages::too_few_args(2, args.len()));
    }

    let value_str = args.last().unwrap().clone();
    let keys: Vec<String> = args[..args.len() - 1]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();

    if keys.is_empty() {
        return FnOutput::error("jsonArrayIndex", crate::error_messages::required(1, "key"));
    }

    let needle = infer_value(&value_str);

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text("-1".to_string()),
        Some(root) => match get_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                let idx = arr.iter().position(|v| v == &needle);
                FnOutput::Text(idx.map(|i| i.to_string()).unwrap_or_else(|| "-1".to_string()))
            }
            Some(_) => FnOutput::error("jsonArrayIndex", "target is not an array"),
            None => FnOutput::Text("-1".to_string()),
        },
    })
}
