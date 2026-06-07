use crate::context::{DiscordContext, FnOutput};
use serenity::builder::CreateInvite;
use serenity::model::id::{ChannelId, RoleId};

/// ZcreateInviteWithRoles{channelID;roleIDs;maxAgeDays?}
/// roleIDs: comma-separated list of role IDs.
/// maxAgeDays: optional, number of days before the invite expires.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createInviteWithRoles", crate::error_messages::required(1, "channelID")),
    };
    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createInviteWithRoles", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };

    let roles_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createInviteWithRoles", crate::error_messages::required(2, "roleIDs")),
    };
    let role_ids: Vec<RoleId> = roles_str
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<u64>().ok().map(RoleId::new)
            }
        })
        .collect();

    let max_age_days: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let max_age_seconds = max_age_days * 86400;

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createInviteWithRoles", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let map = serde_json::json!({
                "max_age": max_age_seconds,
                "role_ids": role_ids
            });

            http.create_invite(ChannelId::new(cid), &map, None)
                .await
                .map(|inv| inv.code)
                .map_err(|e| crate::error_messages::action_failed_reason("create invite", &e.to_string()))
        })
    });

    match result {
        Ok(code) => FnOutput::Text(code),
        Err(e) => FnOutput::error("createInviteWithRoles", e),
    }
}
