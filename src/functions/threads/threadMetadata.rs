use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZthreadMetadata{threadID}
/// Returns JSON with thread metadata: slowmode, creation date, owner ID, member count, archived, locked, flags.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("threadMetadata", crate::error_messages::expected_snowflake(1, "thread ID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadMetadata", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .to_channel(&http)
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(ch) => match ch {
            serenity::model::channel::Channel::Guild(gc) => {
                let owner_id = gc.owner_id.map(|id| id.to_string()).unwrap_or_default();
                let created_at = gc.id.created_at().unix_timestamp().to_string();
                let flags = gc.flags.bits();

                let meta = serde_json::json!({
                    "id": gc.id.to_string(),
                    "name": gc.name,
                    "type": format!("{:?}", gc.kind),
                    "owner_id": owner_id,
                    "created_at": created_at,
                    "rate_limit_per_user": gc.rate_limit_per_user.unwrap_or(0),
                    "parent_id": gc.parent_id.map(|id| id.to_string()).unwrap_or_default(),
                    "flags": flags,
                });
                FnOutput::Text(serde_json::to_string(&meta).unwrap_or_default())
            }
            _ => FnOutput::error("threadMetadata", "not a guild channel"),
        },
        Err(e) => FnOutput::error("threadMetadata", e),
    }
}
