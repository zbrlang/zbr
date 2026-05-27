use crate::context::{DiscordContext, FnOutput};
use crate::bot::parse_reaction_type;
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{ChannelId, MessageId};

/// ZclearReactions{channelID;messageID;emoji}
/// Use "!all" as emoji to clear all reactions.
/// Requires MANAGE_MESSAGES permission.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("clearReactions", crate::error_messages::too_few_args(3, args.len()));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("clearReactions", "no HTTP client available"),
    };

    let channel_id = match validate_snowflake(&args[0], "clearReactions", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let message_id = match validate_snowflake(&args[1], "clearReactions", "message ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let emoji_str = args[2].clone();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(channel_id);
            let msg_id = MessageId::new(message_id);
            if emoji_str == "!all" {
                channel.delete_reactions(&http, msg_id).await
                    .map_err(|e| format!("{}", e))
            } else {
                let reaction = parse_reaction_type(&emoji_str);
                channel.delete_reaction_emoji(&http, msg_id, reaction).await
                    .map_err(|e| format!("{}", e))
            }
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("clearReactions", e),
    }
}
