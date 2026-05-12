use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_at_path};

/// ZjsonGet{key;...}
/// Returns the value at the given key path in the working JSON object.
/// Returns empty string if the key doesn't exist or the value is null.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonGet", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text(String::new()),
        Some(root) => match get_at_path(root, &keys) {
            None | Some(serde_json::Value::Null) => FnOutput::Text(String::new()),
            Some(serde_json::Value::String(s)) => FnOutput::Text(s.clone()),
            Some(other) => FnOutput::Text(other.to_string()),
        },
    })
}
