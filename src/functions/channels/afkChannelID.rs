use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("afkChannelID", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("afkChannelID", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).to_partial_guild(&http).await
        })
    });

    match result {
        Ok(guild) => {
            match guild.afk_metadata {
                Some(m) => FnOutput::Text(m.afk_channel_id.to_string()),
                None => FnOutput::Text("".to_string()),
            }
        }
        Err(_) => FnOutput::error("afkChannelID", crate::error_messages::action_failed("fetch guild")),
    }
}
