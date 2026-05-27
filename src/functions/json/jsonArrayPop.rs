use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path};

/// ZjsonArrayPop{key;...}
/// Removes and returns the last element of the array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArrayPop", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArrayPop", crate::error_messages::requires_first("ZjsonParse or ZjsonArray")),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => match arr.pop() {
                Some(serde_json::Value::String(s)) => FnOutput::Text(s),
                Some(serde_json::Value::Null) | None => FnOutput::Text(String::new()),
                Some(other) => FnOutput::Text(other.to_string()),
            },
            Some(_) => FnOutput::error("jsonArrayPop", "target is not an array"),
            None => FnOutput::error("jsonArrayPop", "key path not found"),
        },
    })
}
