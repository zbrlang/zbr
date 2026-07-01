use crate::context::{DiscordContext, FnOutput};
use super::helpers::{apply_time_placeholders, parse_duration};

/// Zcooldown{duration;(errorMessage)}
/// Per-user per-server cooldown.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let duration_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let error_msg    = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let duration_secs = match parse_duration(&duration_str) {
        Ok(d) => d,
        Err(e) => return FnOutput::error("cooldown", e),
    };

    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("cooldown", crate::error_messages::not_available("database")),
    };

    let bot_id   = ctx.bot_id.clone();
    let guild_id = ctx.guild_id.clone();
    let user_id  = ctx.author_id.clone();
    let command  = ctx.command_name.clone();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            crate::db::try_acquire_user_cooldown(&db, &bot_id, &guild_id, &user_id, &command, duration_secs).await
        })
    });

    match result {
        Ok(None) => FnOutput::Empty,
        Ok(Some(remaining)) => {
            let labels = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    ctx.cooldown_labels.lock().await.clone()
                })
            });
            let msg = if error_msg.is_empty() {
                format!("You are on cooldown. Try again in {}.", super::helpers::format_remaining(remaining, &labels))
            } else {
                apply_time_placeholders(&error_msg, remaining, &labels)
            };
            FnOutput::user_error(msg)
        }
        Err(e) => FnOutput::error("cooldown", crate::error_messages::action_failed_reason("acquire user cooldown", &e.to_string())),
    }
}
