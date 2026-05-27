use crate::context::{DiscordContext, FnOutput};
use crate::bot::parse_reaction_type;
use serenity::model::id::{ChannelId, MessageId};

/// ZaddCmdReactions{emoji1;emoji2;...}
/// Adds reactions to the message that triggered the command.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("addCmdReactions", crate::error_messages::too_few_args(1, args.len()));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("addCmdReactions", "no HTTP client available"),
    };

    let msg_id_str = match &ctx.trigger_message_id {
        Some(id) => id.clone(),
        None => return FnOutput::error("addCmdReactions", "no trigger message available (slash commands don't have a trigger message)"),
    };

    let channel_id: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("addCmdReactions", "invalid channel ID"),
    };

    let msg_id: u64 = match msg_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("addCmdReactions", "invalid trigger message ID"),
    };

    let emojis = args.clone();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(channel_id);
            let message_id = MessageId::new(msg_id);
            for emoji_str in &emojis {
                if emoji_str.is_empty() { continue; }
                let reaction = parse_reaction_type(emoji_str);
                channel.create_reaction(&http, message_id, reaction).await.ok();
            }
        })
    });

    FnOutput::Empty
}
