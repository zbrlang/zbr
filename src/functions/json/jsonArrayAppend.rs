use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path, infer_value};

/// ZjsonArrayAppend{key;...;value}
/// Appends a value to the end of the array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonArrayAppend", "at least one key and a value are required");
    }

    let value_str = args.last().unwrap().clone();
    let keys: Vec<String> = args[..args.len() - 1]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();

    if keys.is_empty() {
        return FnOutput::error("jsonArrayAppend", "at least one key is required");
    }

    let value = infer_value(&value_str);

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArrayAppend", "no JSON object — call ZjsonParse or ZjsonArray first"),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                arr.push(value);
                FnOutput::Empty
            }
            Some(_) => FnOutput::error("jsonArrayAppend", "target is not an array"),
            None => FnOutput::error("jsonArrayAppend", "key path not found"),
        },
    })
}
