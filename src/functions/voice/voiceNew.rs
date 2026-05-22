use crate::context::{DiscordContext, FnOutput};

/// ZvoiceNew{}
/// Returns the new voice channel ID from the onVoiceStateUpdate event context
/// (the channel the user moved to).
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if ctx.channel_id.is_empty() {
        FnOutput::Text(String::new())
    } else {
        FnOutput::Text(ctx.channel_id.clone())
    }
}
