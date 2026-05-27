use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if guild_id_str.is_empty() {
        return FnOutput::Text("0".to_string());
    }

    let guild_id = match guild_id_str.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("afkTimeout", crate::error_messages::not_found("guild", &guild_id_str)),
    };

    let http = match ctx.http.as_ref() {
        Some(h) => h.clone(),
        None => return FnOutput::error("afkTimeout", "no HTTP client available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_guild(guild_id).await {
                Ok(_guild) => FnOutput::Text("0".to_string()),
                Err(_) => FnOutput::error("afkTimeout", crate::error_messages::not_found("guild", &guild_id_str)),
            }
        })
    })
}
