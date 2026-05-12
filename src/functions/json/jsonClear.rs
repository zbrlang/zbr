use crate::context::{DiscordContext, FnOutput};
use super::helpers::with_json;

/// ZjsonClear{}
/// Clears the working JSON object for this execution.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    with_json(ctx, |obj| *obj = None);
    FnOutput::Empty
}
