use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("botTyping", crate::error_messages::requires_set_first("HTTP client")),
    };

    let channel_id: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("botTyping", crate::error_messages::expected_snowflake(1, "channel ID", &ctx.channel_id)),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(channel_id)
                .broadcast_typing(&http)
                .await
                .map_err(|e| e.to_string())
        })
    });

    if let Err(e) = result {
        return FnOutput::error("botTyping", crate::error_messages::action_failed_reason("broadcast typing", &e));
    }

    FnOutput::Empty
}
