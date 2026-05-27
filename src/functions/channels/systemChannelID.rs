use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("systemChannelID", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("systemChannelID", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).to_partial_guild(&http).await
        })
    });

    match result {
        Ok(guild) => {
            match guild.system_channel_id {
                Some(id) => FnOutput::Text(id.to_string()),
                None => FnOutput::Text("".to_string()),
            }
        }
        Err(_) => FnOutput::error("systemChannelID", crate::error_messages::action_failed("fetch guild")),
    }
}
