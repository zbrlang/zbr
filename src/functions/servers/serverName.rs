use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());

    let guild_id = match guild_id_str.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("serverName", crate::error_messages::not_found("guild", &guild_id_str)),
    };

    let http = match ctx.http.as_ref() {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverName", crate::error_messages::action_failed("get HTTP client")),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_guild(guild_id).await {
                Ok(guild) => FnOutput::Text(guild.name.clone()),
                Err(_) => FnOutput::error("serverName", crate::error_messages::not_found("guild", &guild_id_str)),
            }
        })
    })
}
