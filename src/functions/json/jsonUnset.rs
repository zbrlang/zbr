use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, unset_at_path};

/// ZjsonUnset{key;...}
/// Removes the key at the given path from the working JSON object.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonUnset", "at least one key is required");
    }

    with_json(ctx, |obj| {
        if let Some(root) = obj {
            unset_at_path(root, &keys);
        }
    });

    FnOutput::Empty
}
