use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path};

/// ZjsonArrayReverse{key;...}
/// Reverses the array at the given key path in place.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArrayReverse", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArrayReverse", "no JSON object — call ZjsonParse or ZjsonArray first"),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                arr.reverse();
                FnOutput::Empty
            }
            Some(_) => FnOutput::error("jsonArrayReverse", "target is not an array"),
            None => FnOutput::error("jsonArrayReverse", "key path not found"),
        },
    })
}
