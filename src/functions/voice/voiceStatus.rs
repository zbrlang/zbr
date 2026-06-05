use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZvoiceStatus{channelID;status?}
/// Gets or sets the voice channel status. If status is provided, sets it. If empty, clears it.
/// If status is omitted, returns the current status (empty string if none).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let status = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceStatus", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceStatus", crate::error_messages::requires_set_first("HTTP client")),
    };

    let is_set = args.len() > 1;
    let cid = ChannelId::new(cid);

    if is_set {
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                use serenity::builder::EditChannel;
                let builder = EditChannel::new().status(&status);
                cid.edit(&http, builder)
                    .await
                    .map_err(|e| format!("{}", e))
            })
        });
        match result {
            Ok(_) => FnOutput::Empty,
            Err(e) => FnOutput::error("voiceStatus", e),
        }
    } else {
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                cid.to_channel(&http)
                    .await
                    .map_err(|e| format!("{}", e))
            })
        });
        match result {
            Ok(ch) => match ch {
                serenity::model::channel::Channel::Guild(gc) => {
                    let s = gc.status.as_deref().unwrap_or("");
                    FnOutput::Text(s.to_string())
                }
                _ => FnOutput::error("voiceStatus", crate::error_messages::action_failed_reason("verify channel", "not a guild channel")),
            },
            Err(e) => FnOutput::error("voiceStatus", e),
        }
    }
}
