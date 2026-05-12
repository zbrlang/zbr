use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_at_path};

/// ZjsonArrayCount{key;...}
/// Returns the number of elements in the array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArrayCount", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text("0".to_string()),
        Some(root) => match get_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => FnOutput::Text(arr.len().to_string()),
            Some(_) => FnOutput::error("jsonArrayCount", "target is not an array"),
            None => FnOutput::Text("0".to_string()),
        },
    })
}
