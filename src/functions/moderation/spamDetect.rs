use crate::context::{ DiscordContext, FnOutput };

/// ZspamDetect{userID;threshold?;windowSeconds?}
/// Logs the current message and checks if the user has exceeded the threshold.
/// Returns "true" if spam detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("spamDetect", crate::error_messages::required(1, "userID"));
        }
    };

    let threshold: i64 = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "spamDetect",
                        crate::error_messages::expected_integer(2, "threshold", s)
                    );
                }
            }
        _ => 10,
    };

    let window_seconds: i64 = match args.get(2) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "spamDetect",
                        crate::error_messages::expected_integer(3, "windowSeconds", s)
                    );
                }
            }
        _ => 60,
    };

    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => {
            return FnOutput::error("spamDetect", crate::error_messages::not_available("database"));
        }
    };

    let bot_id = ctx.bot_id.clone();
    let guild_id = ctx.guild_id.clone();
    let channel_id = ctx.channel_id.clone();
    let message = ctx.message.clone();

    if guild_id.is_empty() {
        return FnOutput::error("spamDetect", crate::error_messages::not_in_guild());
    }

    let has_link = message.contains("http://") || message.contains("https://");

    let count = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            crate::db::log_spam_event(
                &db,
                &bot_id,
                &guild_id,
                &user_id,
                &channel_id,
                has_link
            ).await;
            crate::db::get_spam_count(&db, &bot_id, &guild_id, &user_id, window_seconds).await
        })
    });

    FnOutput::Text((if count >= threshold { "true" } else { "false" }).to_string())
}
