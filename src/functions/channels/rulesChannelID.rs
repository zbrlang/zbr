use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("rulesChannelID", crate::error_messages::expected_snowflake(1, "guild ID", &gid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("rulesChannelID", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).to_partial_guild(&http).await
        })
    });

    match result {
        Ok(guild) => {
            match guild.rules_channel_id {
                Some(id) => FnOutput::Text(id.to_string()),
                None => FnOutput::Text("".to_string()),
            }
        }
        Err(_) => FnOutput::error("rulesChannelID", crate::error_messages::action_failed("fetch guild")),
    }
}
