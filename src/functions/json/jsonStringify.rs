use crate::context::{DiscordContext, FnOutput};
use super::helpers::with_json;

/// ZjsonStringify{}
/// Serializes the working JSON object to a compact JSON string.
/// Returns empty string if no object is set.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    with_json(ctx, |obj| match obj {
        None => FnOutput::Text(String::new()),
        Some(v) => FnOutput::Text(v.to_string()),
    })
}
