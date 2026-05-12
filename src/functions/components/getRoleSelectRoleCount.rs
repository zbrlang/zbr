use crate::context::{DiscordContext, FnOutput};

/// ZgetRoleSelectRoleCount{} — returns the number of selected roles.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.selected_values.len().to_string())
}
