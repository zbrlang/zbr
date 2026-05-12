use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, set_at_path, infer_value};

/// ZjsonSet{key;...;value}
/// Sets a value at the given key path. Auto-detects type (bool, number, string).
/// At least 2 args required: one key and one value.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonSet", "at least one key and a value are required");
    }

    let value_str = args.last().unwrap().clone();
    let keys: Vec<String> = args[..args.len() - 1]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();

    if keys.is_empty() {
        return FnOutput::error("jsonSet", "at least one key is required");
    }

    let value = infer_value(&value_str);

    with_json(ctx, |obj| {
        let root = obj.get_or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        set_at_path(root, &keys, value);
    });

    FnOutput::Empty
}
