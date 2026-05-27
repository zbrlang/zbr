use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("deleteChannelsByName", crate::error_messages::too_few_args(1, 0));
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteChannelsByName", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteChannelsByName", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channels = GuildId::new(gid).channels(&http).await.map_err(|e| crate::error_messages::action_failed_reason("fetch channels", &e.to_string()))?;
            
            for arg in args {
                let target_name = arg.to_lowercase();
                let mut found = false;
                for c in channels.values() {
                    if c.name.to_lowercase() == target_name {
                        found = true;
                        if let Err(_) = c.delete(&http).await {
                            return Err(crate::error_messages::action_failed_reason("delete channel", &c.name));
                        }
                    }
                }
                if !found {
                    return Err(crate::error_messages::not_found("channel", &arg));
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
