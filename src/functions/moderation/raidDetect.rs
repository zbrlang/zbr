use crate::context::{ DiscordContext, FnOutput };

/// ZraidDetect{joinCount;windowSeconds?}
/// Logs the current member join and checks if the guild has exceeded the join threshold.
/// Returns "true" if raid detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let join_count: i64 = match args.get(0) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "raidDetect",
                        crate::error_messages::expected_integer(1, "joinCount", s)
                    );
                }
            }
        _ => {
            return FnOutput::error("raidDetect", crate::error_messages::required(1, "joinCount"));
        }
    };

    let window_seconds: i64 = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "raidDetect",
                        crate::error_messages::expected_integer(2, "windowSeconds", s)
                    );
                }
            }
        _ => 60,
    };

    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => {
            return FnOutput::error("raidDetect", crate::error_messages::not_available("database"));
        }
    };

    let bot_id = ctx.bot_id.clone();
    let guild_id = ctx.guild_id.clone();
    let user_id = ctx.author_id.clone();

    if guild_id.is_empty() {
        return FnOutput::error("raidDetect", crate::error_messages::not_in_guild());
    }

    let count = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            crate::db::log_raid_event(&db, &bot_id, &guild_id, &user_id).await;
            crate::db::get_raid_count(&db, &bot_id, &guild_id, window_seconds).await
        })
    });

    FnOutput::Text((if count >= join_count { "true" } else { "false" }).to_string())
}
