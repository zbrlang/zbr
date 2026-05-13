use crate::context::{DiscordContext, FnOutput};

/// ZmessageLink{messageID;channelID?}
/// Builds a Discord jump URL. channelID defaults to the current channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let message_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("messageLink", "messageID is required"),
    };
    let channel_id = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let guild_id = &ctx.guild_id;
    FnOutput::Text(format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id, channel_id, message_id
    ))
}
