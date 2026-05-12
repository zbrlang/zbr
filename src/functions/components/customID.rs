use crate::context::{DiscordContext, FnOutput};

/// ZcustomID{} — returns the custom_id of the current interaction.
/// Only populated inside onInteraction handlers.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.custom_id.clone().unwrap_or_default())
}
