use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, set_at_path};

/// ZjsonArray{key;...}
/// Creates an empty array at the given key path.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonArray", crate::error_messages::required(1, "key"));
    }

    with_json(ctx, |obj| {
        let root = obj.get_or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        set_at_path(root, &keys, serde_json::Value::Array(vec![]));
    });

    FnOutput::Empty
}
