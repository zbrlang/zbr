use crate::context::{DiscordContext, FnOutput};

/// ZuseChannel{channelID}
/// Redirects all bot output for this execution to the specified channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("useChannel", "channelID is required"),
    };

    // Validate it's a numeric snowflake
    if cid_str.parse::<u64>().is_err() {
        return FnOutput::error("useChannel", format!("invalid channel ID: '{}'", cid_str));
    }

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.use_channel.lock().await = Some(cid_str);
        })
    });

    FnOutput::Empty
}
