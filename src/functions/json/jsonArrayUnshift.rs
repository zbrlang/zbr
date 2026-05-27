use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_mut_at_path, infer_value};

/// ZjsonArrayUnshift{key;...;value}
/// Inserts a value at the front of the array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonArrayUnshift", "at least one key and a value are required");
    }

    let value_str = args.last().unwrap().clone();
    let keys: Vec<String> = args[..args.len() - 1]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();

    if keys.is_empty() {
        return FnOutput::error("jsonArrayUnshift", "at least one key is required");
    }

    let value = infer_value(&value_str);

    with_json(ctx, |obj| match obj {
        None => FnOutput::error("jsonArrayUnshift", crate::error_messages::requires_first("ZjsonParse or ZjsonArray")),
        Some(root) => match get_mut_at_path(root, &keys) {
            Some(serde_json::Value::Array(arr)) => {
                arr.insert(0, value);
                FnOutput::Empty
            }
            Some(_) => FnOutput::error("jsonArrayUnshift", "target is not an array"),
            None => FnOutput::error("jsonArrayUnshift", "key path not found"),
        },
    })
}
