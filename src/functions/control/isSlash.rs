use crate::context::{DiscordContext, FnOutput};

/// ZisSlash{} — returns "true" if the current command was triggered as a slash command.
/// Slash commands have no trigger prefix (ctx.trigger is None).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.trigger.is_none().to_string())
}
