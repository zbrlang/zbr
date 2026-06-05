use crate::context::{DiscordContext, FnOutput};

/// ZgetCooldown{type}
/// Returns remaining seconds on the cooldown.
/// type: "normal" | "server" | "global"
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let kind = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "normal".to_string());

    let db = match &ctx.db {
        Some(d) => d.clone(),
        None => return FnOutput::error("getCooldown", crate::error_messages::not_available("database")),
    };

    let bot_id   = ctx.bot_id.clone();
    let guild_id = ctx.guild_id.clone();
    let user_id  = ctx.author_id.clone();
    let command  = ctx.command_name.clone();

    let remaining = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match kind.as_str() {
                "server" => crate::db::get_server_cooldown(&db, &bot_id, &guild_id, &command).await,
                "global" => crate::db::get_global_cooldown(&db, &bot_id, &user_id, &command).await,
                _        => crate::db::get_user_cooldown(&db, &bot_id, &guild_id, &user_id, &command).await,
            }
        })
    });

    FnOutput::Text(remaining.to_string())
}
