use crate::context::{DiscordContext, FnOutput};
use crate::bot::parse_reaction_type;
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{ChannelId, MessageId};

/// ZgetReactions{channelID;messageID;separator;emoji}
/// Returns a list of usernames who reacted with the given emoji.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 4 {
        return FnOutput::error("getReactions", "requires channelID, messageID, separator, and emoji");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("getReactions", "no HTTP client available"),
    };

    let channel_id = match validate_snowflake(&args[0], "getReactions", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let message_id = match validate_snowflake(&args[1], "getReactions", "message ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let separator = args[2].clone();
    let emoji_str = args[3].clone();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let reaction = parse_reaction_type(&emoji_str);
            ChannelId::new(channel_id)
                .reaction_users(&http, MessageId::new(message_id), reaction, None, None)
                .await
                .map(|users| {
                    users.iter()
                        .map(|u| u.name.clone())
                        .collect::<Vec<_>>()
                        .join(&separator)
                })
                .map_err(|e| format!("failed to get reactions: {}", e))
        })
    });

    match result {
        Ok(list) => FnOutput::Text(list),
        Err(e) => FnOutput::error("getReactions", e),
    }
}
