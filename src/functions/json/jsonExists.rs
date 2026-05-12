use crate::context::{DiscordContext, FnOutput};
use super::helpers::{with_json, get_at_path};

/// ZjsonExists{key;...}
/// Returns "true" if the key path exists in the working JSON object, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let keys: Vec<String> = args.into_iter().filter(|s| !s.is_empty()).collect();
    if keys.is_empty() {
        return FnOutput::error("jsonExists", "at least one key is required");
    }

    with_json(ctx, |obj| match obj {
        None => FnOutput::Text("false".to_string()),
        Some(root) => FnOutput::Text(get_at_path(root, &keys).is_some().to_string()),
    })
}
