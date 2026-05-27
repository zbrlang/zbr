use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZchannelInvites{channelID?} — space-separated list of invite codes for a channel
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelInvites", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelInvites", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .invites(&http)
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(invites) => FnOutput::Text(
            invites
                .iter()
                .map(|i| i.code.clone())
                .collect::<Vec<_>>()
                .join(" "),
        ),
        Err(e) => FnOutput::error("channelInvites", e),
    }
}
