use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::channel::ChannelType;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid_str = args.get(0).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if gid_str.is_empty() {
        return FnOutput::error("categoryCount", "invalid guild ID");
    }

    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("categoryCount", "invalid guild ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("categoryCount", "no HTTP client available"),
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
        Err(_) => FnOutput::error("categoryCount", "failed to fetch channels"),
    }
}
