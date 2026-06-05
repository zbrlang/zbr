use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "\n".to_string());
    
    let gid_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if gid_str.is_empty() {
        return FnOutput::error("channelNames", crate::error_messages::required(2, "guild ID"));
    }

    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelNames", crate::error_messages::expected_snowflake(2, "guild ID", &gid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelNames", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).channels(&http).await
        })
    });

    match result {
        Ok(channels) => {
            let mut names: Vec<String> = channels.values().map(|c| c.name.clone()).collect();
            names.sort();
            FnOutput::Text(names.join(&separator))
        }
        Err(_) => FnOutput::error("channelNames", crate::error_messages::action_failed("fetch channels")),
    }
}
