use crate::context::{DiscordContext, FnOutput};
use serde_json::json;
use serenity::http::Http;
use serenity::model::guild::audit_log::{AuditLogEntry, AuditLogs};
use serenity::model::id::{AuditLogEntryId, GuildId};
use std::sync::Arc;

pub fn parse_guild_id(args: &[String], ctx: &DiscordContext, function: &str) -> Result<GuildId, FnOutput> {
    let guild_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| ctx.guild_id.clone());
    let guild_id_str = guild_id_str.trim();
    if guild_id_str.is_empty() {
        return Err(FnOutput::error(function, "guild ID not found"));
    }

    match guild_id_str.parse::<u64>() {
        Ok(id) => Ok(GuildId::new(id)),
        Err(_) => Err(FnOutput::error(function, "invalid guild ID")),
    }
}

pub fn parse_limit(args: &[String], minimum: u8) -> u8 {
    args.get(1)
        .and_then(|limit| limit.parse::<u8>().ok())
        .filter(|&limit| limit >= minimum)
        .unwrap_or(minimum)
}

pub fn http_client(ctx: &DiscordContext, function: &str) -> Result<Arc<Http>, FnOutput> {
    match &ctx.http {
        Some(http) => Ok(http.clone()),
        None => Err(FnOutput::error(function, "no HTTP client available")),
    }
}

pub fn fetch_audit_logs(
    guild_id: GuildId,
    http: Arc<Http>,
    limit: Option<u8>,
    function: &str,
) -> Result<AuditLogs, FnOutput> {
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            guild_id
                .audit_logs(&http, None, None, None, limit)
                .await
                .map_err(|e| e.to_string())
        })
    });

    result.map_err(|e| FnOutput::error(function, e))
}

pub fn entry_summary(entry: &AuditLogEntry) -> serde_json::Value {
    json!({
        "id": entry.id.to_string(),
        "user_id": entry.user_id.to_string(),
        "action": format!("{:?}", entry.action),
        "action_code": entry.action.num(),
        "target_id": entry.target_id.as_ref().map(|id| id.to_string()),
        "reason": entry.reason.clone().unwrap_or_default(),
        "changes": entry.changes.clone().unwrap_or_default(),
    })
}

pub fn select_entry<'a>(
    entries: &'a [AuditLogEntry],
    selector: Option<&String>,
    function: &str,
) -> Result<&'a AuditLogEntry, FnOutput> {
    match selector {
        Some(selector) if !selector.trim().is_empty() => {
            if let Ok(index) = selector.parse::<usize>() {
                let idx = index.saturating_sub(1);
                entries
                    .get(idx)
                    .ok_or_else(|| FnOutput::error(function, "audit entry not found"))
            } else if let Ok(entry_id) = selector.parse::<AuditLogEntryId>() {
                entries
                    .iter()
                    .find(|entry| entry.id == entry_id)
                    .ok_or_else(|| FnOutput::error(function, "audit entry not found"))
            } else {
                Err(FnOutput::error(function, "invalid audit entry selector"))
            }
        }
        _ => entries
            .first()
            .ok_or_else(|| FnOutput::error(function, "no audit entries found")),
    }
}
