use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("deleteChannelsByName", "at least one name is required");
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteChannelsByName", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteChannelsByName", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channels = GuildId::new(gid).channels(&http).await.map_err(|e| format!("failed to fetch channels: {}", e))?;
            
            for arg in args {
                let target_name = arg.to_lowercase();
                let mut found = false;
                for c in channels.values() {
                    if c.name.to_lowercase() == target_name {
                        found = true;
                        if let Err(_) = c.delete(&http).await {
                            return Err(format!("failed to delete channel: '{}'", c.name));
                        }
                    }
                }
                if !found {
                    return Err(format!("channel not found: '{}'", arg));
                }
            }
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("deleteChannelsByName", e),
    }
}
