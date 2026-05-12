use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).cloned().unwrap_or_else(|| "\n".to_string());
    
    let gid_str = args.get(1).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    if gid_str.is_empty() {
        return FnOutput::error("channelNames", "invalid guild ID");
    }

    let gid: u64 = match gid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelNames", "invalid guild ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelNames", "no HTTP client available"),
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
        Err(_) => FnOutput::error("channelNames", "failed to fetch channels"),
    }
}
