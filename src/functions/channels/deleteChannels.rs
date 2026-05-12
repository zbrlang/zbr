use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("deleteChannels", "at least one channel ID is required");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteChannels", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            for arg in args {
                let cid: u64 = match arg.parse() {
                    Ok(id) => id,
                    Err(_) => return Err(format!("invalid channel ID: '{}'", arg)),
                };
                
                if let Err(_) = ChannelId::new(cid).delete(&http).await {
                    return Err(format!("channel not found: '{}'", arg));
                }
            }
            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("deleteChannels", e),
    }
}
