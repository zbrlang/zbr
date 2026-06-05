use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("categoryID", crate::error_messages::required(1, "name"));
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryID", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("categoryID", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).channels(&http).await
        })
    });

    match result {
        Ok(channels) => {
            for (_, c) in channels {
                if c.kind == ChannelType::Category && c.name.to_lowercase() == name.to_lowercase() {
                    return FnOutput::Text(c.id.to_string());
                }
            }
            FnOutput::error("categoryID", crate::error_messages::not_found("category", &name))
        }
        Err(_) => FnOutput::error("categoryID", crate::error_messages::action_failed("fetch channels")),
    }
}
