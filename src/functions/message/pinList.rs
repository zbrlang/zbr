use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZpinList{channelID?}
/// Returns a space-separated list of pinned message IDs in the given channel (or current channel).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.channel_id.clone());

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("pinList", "invalid channel ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("pinList", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .pins(&http)
                .await
                .map(|msgs| {
                    msgs.iter()
                        .map(|m| m.id.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(ids) => FnOutput::Text(ids),
        Err(e) => FnOutput::error("pinList", e),
    }
}
