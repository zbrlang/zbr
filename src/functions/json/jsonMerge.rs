use crate::context::{DiscordContext, FnOutput};
use serde_json::Value;
use super::helpers::{with_json, get_mut_at_path, set_at_path};

fn deep_merge(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                deep_merge(a_map.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

/// ZjsonMerge{target;sourceJson}
/// Deep-merges the parsed sourceJson into the value at the target key path.
/// All args except the last form the target path; the last is a JSON string.
/// Objects are merged recursively; scalars and arrays overwrite.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("jsonMerge", crate::error_messages::required(1, "target"));
    }

    let source = args.last().unwrap();
    let keys: Vec<String> = args[..args.len() - 1].iter().filter(|s| !s.is_empty()).cloned().collect();

    let merge_value: Value = match serde_json::from_str(source) {
        Ok(v) => v,
        Err(e) => return FnOutput::error("jsonMerge", format!("invalid JSON: {}", e)),
    };

    with_json(ctx, |obj| {
        let root = obj.get_or_insert_with(|| Value::Object(serde_json::Map::new()));

        if keys.is_empty() {
            deep_merge(root, merge_value);
        } else {
            let target = get_mut_at_path(root, &keys);
            match target {
                Some(val) => deep_merge(val, merge_value),
                None => {
                    set_at_path(root, &keys, merge_value);
                }
            }
        }
    });

    FnOutput::Empty
}
