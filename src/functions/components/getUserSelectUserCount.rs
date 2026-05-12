use crate::context::{DiscordContext, FnOutput};

/// ZgetUserSelectUserCount{} — returns the number of selected users.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.selected_values.len().to_string())
}
