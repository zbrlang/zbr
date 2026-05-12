use crate::context::{DiscordContext, FnOutput};
use crate::bot::parse_reaction_type;
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{ChannelId, MessageId};

/// ZaddMessageReactions{channelID;messageID;emoji1;emoji2;...}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("addMessageReactions", "requires channelID, messageID, and at least one emoji");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("addMessageReactions", "no HTTP client available"),
    };

    let channel_id = match validate_snowflake(&args[0], "addMessageReactions", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let message_id = match validate_snowflake(&args[1], "addMessageReactions", "message ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let emojis = args[2..].to_vec();
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(channel_id);
            let msg_id = MessageId::new(message_id);
            for emoji_str in &emojis {
                if emoji_str.is_empty() { continue; }
                let reaction = parse_reaction_type(emoji_str);
                channel.create_reaction(&http, msg_id, reaction).await.ok();
            }
        })
    });

    FnOutput::Empty
}
