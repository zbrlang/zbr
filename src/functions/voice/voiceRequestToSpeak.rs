use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{Builder, EditVoiceState};
use serenity::model::id::{ChannelId, GuildId};

/// ZvoiceRequestToSpeak{channelID;cancel?}
/// Requests to speak in the given stage channel, or cancels the request if cancel=true.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_default();
    let cancel = args
        .get(1)
        .map(|s| s.to_lowercase() == "true")
        .unwrap_or(false);

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceRequestToSpeak", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceRequestToSpeak", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceRequestToSpeak", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditVoiceState::new().suppress(false).request_to_speak(!cancel);
            builder
                .execute(&http, (GuildId::new(gid), ChannelId::new(cid), None))
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("voiceRequestToSpeak", e),
    }
}
