use crate::context::{DiscordContext, FnOutput};

/// ZmessageID{} — returns the triggering message ID (prefix commands only).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    FnOutput::Text(ctx.trigger_message_id.clone().unwrap_or_default())
}
