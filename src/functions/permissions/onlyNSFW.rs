use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZonlyNSFW{(errorMessage)}
/// Halts unless the current channel is marked as NSFW.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let error_msg = args.get(0).cloned()
        .unwrap_or_else(|| "This command can only be used in NSFW channels.".to_string());

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyNSFW", "no HTTP client available"),
    };

    let channel_id: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyNSFW", crate::error_messages::expected_snowflake(1, "channel ID", &ctx.channel_id)),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(channel_id).to_channel(&http).await
                .map_err(|e| format!("failed to fetch channel: {}", e))?;
            let is_nsfw = channel.guild().map(|c| c.nsfw).unwrap_or(false);
            Ok::<bool, String>(is_nsfw)
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyNSFW", e),
    }
}
