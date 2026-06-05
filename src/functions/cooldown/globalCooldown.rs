use crate::context::{DiscordContext, FnOutput};
use super::helpers::{apply_time_placeholders, parse_duration};

/// ZglobalCooldown{duration;(errorMessage)}
/// Per-user cooldown across all servers.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let duration_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let error_msg    = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let duration_secs = match parse_duration(&duration_str) {
        Ok(d) => d,
        Err(e) => return FnOutput::error("globalCooldown", e),
    };

    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("globalCooldown", crate::error_messages::not_available("database")),
    };

    let bot_id  = ctx.bot_id.clone();
    let user_id = ctx.author_id.clone();
    let command = ctx.command_name.clone();

    let remaining = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            crate::db::get_global_cooldown(&db, &bot_id, &user_id, &command).await
        })
    });

    if remaining > 0 {
        let labels = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                ctx.cooldown_labels.lock().await.clone()
            })
        });
        let msg = if error_msg.is_empty() {
            format!("You are on global cooldown. Try again in {}.", super::helpers::format_remaining(remaining, &labels))
        } else {
            apply_time_placeholders(&error_msg, remaining, &labels)
        };
        return FnOutput::user_error(msg);
    }

    let res = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            crate::db::set_global_cooldown(&db, &bot_id, &user_id, &command, duration_secs).await
        })
    });
    if let Err(e) = res {
        return FnOutput::error("globalCooldown", crate::error_messages::action_failed_reason("set global cooldown", &e.to_string()));
    }

    FnOutput::Empty
}
