use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelCount", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelCount", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).channels(&http).await
        })
    });

    match result {
        Ok(channels) => FnOutput::Text(channels.len().to_string()),
        Err(_) => FnOutput::error("channelCount", "failed to fetch channels"),
    }
}
