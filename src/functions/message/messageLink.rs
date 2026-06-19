use crate::context::{DiscordContext, FnOutput};

/// ZmessageLink{channelID;messageID}
/// Builds a Discord jump URL. channelID defaults to the current channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let (channel_id, message_id) = if args.len() >= 2 {
        (args.get(0).unwrap().clone(), args.get(1).unwrap().clone())
    } else if let Some(msg_id) = args.get(0) {
        (ctx.channel_id.clone(), msg_id.clone())
    } else {
        return FnOutput::error("messageLink", crate::error_messages::required(1, "messageID"));
    };

    let guild_id = &ctx.guild_id;
    FnOutput::Text(format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id, channel_id, message_id
    ))
}
