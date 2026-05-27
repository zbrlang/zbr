use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid_str = args.get(0).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if gid_str.is_empty() {
        return FnOutput::error("categoryCount", crate::error_messages::required(1, "guild ID"));
    }

    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryCount", crate::error_messages::expected_snowflake(1, "guild ID", &gid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("categoryCount", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).channels(&http).await
        })
    });

    match result {
        Ok(channels) => {
            let count = channels.values().filter(|c| c.kind == ChannelType::Category).count();
            FnOutput::Text(count.to_string())
        }
        Err(_) => FnOutput::error("categoryCount", crate::error_messages::action_failed("fetch channels")),
    }
}
