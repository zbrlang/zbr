use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZstageTopic{channelID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("stageTopic", "channelID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("stageTopic", "invalid channel ID"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("stageTopic", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_stage_instance(ChannelId::new(cid))
                .await
                .map_err(|e| format!("{}", e))
        })
    });
    match result {
        Ok(si) => FnOutput::Text(si.topic),
        Err(e) => FnOutput::error("stageTopic", e),
    }
}
