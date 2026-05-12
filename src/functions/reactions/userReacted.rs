use crate::context::{DiscordContext, FnOutput};
use crate::bot::parse_reaction_type;
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::model::id::{ChannelId, MessageId, UserId};

/// ZuserReacted{channelID;messageID;userID;emoji}
/// Returns "true" if the user reacted with the emoji, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 4 {
        return FnOutput::error("userReacted", "requires channelID, messageID, userID, and emoji");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userReacted", "no HTTP client available"),
    };

    let channel_id = match validate_snowflake(&args[0], "userReacted", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let message_id = match validate_snowflake(&args[1], "userReacted", "message ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let user_id = match validate_snowflake(&args[2], "userReacted", "user ID") {
        Ok(id) => id, Err(e) => return e,
    };
    let emoji_str = args[3].clone();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let reaction = parse_reaction_type(&emoji_str);
            ChannelId::new(channel_id)
                .reaction_users(&http, MessageId::new(message_id), reaction, None, None)
                .await
                .map(|users| users.iter().any(|u| u.id == UserId::new(user_id)))
                .map_err(|e| format!("failed to get reactions: {}", e))
        })
    });

    match result {
        Ok(reacted) => FnOutput::Text(reacted.to_string()),
        Err(e) => FnOutput::error("userReacted", e),
    }
}
