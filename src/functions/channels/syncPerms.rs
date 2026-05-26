use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZsyncPerms{channelID}
/// Syncs a channel's permissions with its parent category.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).cloned().unwrap_or_default();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("syncPerms", "invalid channel ID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("syncPerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(cid)
                .to_channel(&http)
                .await
                .map_err(|e| format!("failed to fetch channel: {}", e))?;

            match channel {
                serenity::model::channel::Channel::Guild(gc) => {
                    let parent_id = match gc.parent_id {
                        Some(id) => id,
                        None => return Err("channel has no parent category".to_string()),
                    };

                    let parent = parent_id
                        .to_channel(&http)
                        .await
                        .map_err(|e| format!("failed to fetch parent: {}", e))?;

                    let perms = match parent {
                        serenity::model::channel::Channel::Guild(pc) => pc.permission_overwrites,
                        _ => return Err("parent is not a guild channel".to_string()),
                    };

                    let perms_json = serde_json::to_value(&perms)
                        .map_err(|e| format!("failed to serialize perms: {}", e))?;

                    let perms_map = serde_json::json!({"permission_overwrites": perms_json});
                    http.as_ref()
                        .edit_channel(ChannelId::new(cid), &perms_map, None)
                        .await
                        .map_err(|e| format!("failed to sync permissions: {}", e))
                        .map(|_| ())
                }
                _ => Err("not a guild channel".to_string()),
            }
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("syncPerms", e),
    }
}
