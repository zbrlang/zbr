use crate::context::{DiscordContext, FnOutput};

/// ZgetMentionableSelectUserCount{} — returns the number of selected users/roles.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.selected_values.len().to_string())
}
