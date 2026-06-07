use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZchannelOverwrites{channelID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("channelOverwrites", crate::error_messages::required(1, "channelID")),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("channelOverwrites", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("channelOverwrites", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(cid).to_channel(&http).await
                .map_err(|e| crate::error_messages::action_failed_reason("fetch channel", &format!("{}", e)))?;
            
            let guild_channel = match channel.guild() {
                Some(gc) => gc,
                None => return Err("not a guild channel".to_string()),
            };

            let overwrites: Vec<serde_json::Value> = guild_channel.permission_overwrites.iter().map(|o| {
                let kind = match o.kind {
                    serenity::model::channel::PermissionOverwriteType::Role(_) => "role",
                    serenity::model::channel::PermissionOverwriteType::Member(_) => "member",
                    _ => "unknown",
                };
                let id = match o.kind {
                    serenity::model::channel::PermissionOverwriteType::Role(id) => id.to_string(),
                    serenity::model::channel::PermissionOverwriteType::Member(id) => id.to_string(),
                    _ => "0".to_string(),
                };
                serde_json::json!({
                    "id": id,
                    "type": kind,
                    "allow": o.allow.bits(),
                    "deny": o.deny.bits()
                })
            }).collect();

            Ok::<String, String>(serde_json::to_string(&overwrites).unwrap_or_default())
        })
    });

    match result {
        Ok(json) => FnOutput::Text(json),
        Err(e) => FnOutput::error("channelOverwrites", e),
    }
}
