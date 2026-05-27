use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path};

/// ZjsonArrayShift{key;...}
/// Removes and returns the first element of the array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArrayShift", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArrayShift", crate::error_messages::requires_first("ZjsonParse or ZjsonArray")),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                if arr.is_empty() {
                    return FnOutput::Text(String::new());
                }
                let removed = arr.remove(0);
                match removed {
                    serde_json::Value::String(s) => FnOutput::Text(s),
                    serde_json::Value::Null => FnOutput::Text(String::new()),
                    other => FnOutput::Text(other.to_string()),
                }
            }
            Some(_) => FnOutput::error("jsonArrayShift", "target is not an array"),
            None => FnOutput::error("jsonArrayShift", "key path not found"),
        },
    })
}
