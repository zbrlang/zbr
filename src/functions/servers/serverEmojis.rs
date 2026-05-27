use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = if args.len() >= 1 && !args[0].is_empty() {
        args[0].clone()
    } else {
        ctx.guild_id.clone()
    };
    let separator = args.get(1).cloned().unwrap_or_else(|| "\n".to_string());

    if guild_id_str.is_empty() {
        return FnOutput::Text(String::new());
    }

    let guild_id = match guild_id_str.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("serverEmojis", crate::error_messages::not_found("guild", &guild_id_str)),
    };

    let http = match ctx.http.as_ref() {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverEmojis", crate::error_messages::action_failed("get HTTP client")),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_guild(guild_id).await {
                Ok(guild) => {
                    let mut emojis = Vec::new();
                    for emoji in guild.emojis.values() {
                        emojis.push(emoji.to_string());
                    }
                    FnOutput::Text(emojis.join(&separator))
                }
                Err(_) => FnOutput::error("serverEmojis", crate::error_messages::not_found("guild", &guild_id_str)),
            }
        })
    })
}
