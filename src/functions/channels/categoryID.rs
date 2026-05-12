use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("categoryID", "name is required");
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryID", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("categoryID", "no HTTP client available"),
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
            FnOutput::error("categoryID", "category not found")
        }
        Err(_) => FnOutput::error("categoryID", "failed to fetch channels"),
    }
}
