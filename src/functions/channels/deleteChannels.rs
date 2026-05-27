use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("deleteChannels", crate::error_messages::too_few_args(1, 0));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteChannels", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            for arg in args {
                let cid: u64 = match arg.parse() {
                    Ok(id) => id,
                    Err(_) => return Err(crate::error_messages::expected_snowflake(1, "channel ID", &arg)),
                };
                
                if let Err(_) = ChannelId::new(cid).delete(&http).await {
                    return Err(crate::error_messages::not_found("channel", &arg));
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
