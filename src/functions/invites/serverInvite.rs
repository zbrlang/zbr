use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if guild_id_str.is_empty() {
        return FnOutput::Text(String::new());
    }

    let guild_id = match guild_id_str.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("serverInvite", crate::error_messages::not_found("guild", &guild_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverInvite", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let invites = match http.get_guild_invites(guild_id).await {
                Ok(i) => i,
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("Missing Permissions") || err_str.contains("403") {
                        return Err(crate::error_messages::missing_permission("MANAGE_GUILD"));
                    }
                    return Err(crate::error_messages::action_failed("fetch invites"));
                }
            };
            
            Ok(invites.first().map(|i| i.url()).unwrap_or_default())
        })
    });

    match result {
        Ok(url) => FnOutput::Text(url),
        Err(e) => FnOutput::error("serverInvite", e),
    }
}
