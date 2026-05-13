use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateInvite;
use serenity::model::id::ChannelId;

/// ZcreateInvite{channelID;maxUses?;maxAge?}
/// maxUses: 0 = unlimited (default). maxAge: seconds, 0 = never expires (default).
/// Returns the invite code.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createInvite", "channelID is required"),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createInvite", "invalid channel ID"),
    };
    let max_uses: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let max_age: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createInvite", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = CreateInvite::new().max_uses(max_uses).max_age(max_age);
            ChannelId::new(cid)
                .create_invite(&http, builder)
                .await
                .map(|inv| inv.code)
                .map_err(|e| format!("failed to create invite: {}", e))
        })
    });
    match result {
        Ok(code) => FnOutput::Text(code),
        Err(e) => FnOutput::error("createInvite", e),
    }
}
