use crate::context::{DiscordContext, FnOutput};
use chrono::Utc;
use serenity::model::id::GuildId;
use serenity::model::Timestamp;

/// ZserverLockdown{enabled;duration?}
/// Enables or disables guild raid protection (incident actions).
/// enabled: true/false. If enabling, optional duration like "1h", "30m" (default 1h).
/// Disables DMs and invites during the lockdown period.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let enabled_str = args
        .get(0)
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let duration_str = args.get(1).cloned().unwrap_or_else(|| "1h".to_string());
    let duration_clone = duration_str.clone();

    let enabled = match enabled_str.as_str() {
        "true" => true,
        "false" => false,
        _ => return FnOutput::error("serverLockdown", crate::error_messages::expected_boolean(1, "enabled", &enabled_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("serverLockdown", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverLockdown", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut map = serde_json::Map::new();

            if enabled {
                let secs = crate::functions::cooldown::helpers::parse_duration(&duration_clone)
                    .map_err(|_| crate::error_messages::expected_duration(2, "duration", &duration_clone))?;
                let until = Utc::now() + chrono::Duration::seconds(secs);
                let ts = Timestamp::from_unix_timestamp(until.timestamp())
                    .map_err(|_| "failed to compute timestamp".to_string())?;

                map.insert(
                    "invites_disabled_until".to_string(),
                    serde_json::Value::String(ts.to_string()),
                );
                map.insert(
                    "dms_disabled_until".to_string(),
                    serde_json::Value::String(ts.to_string()),
                );
            } else {
                map.insert(
                    "invites_disabled_until".to_string(),
                    serde_json::Value::Null,
                );
                map.insert(
                    "dms_disabled_until".to_string(),
                    serde_json::Value::Null,
                );
            }

            http.as_ref()
                .edit_guild_incident_actions(GuildId::new(gid), &map)
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => {
            if enabled {
                FnOutput::Text(format!("Lockdown enabled for {}", duration_str))
            } else {
                FnOutput::Text("Lockdown disabled".to_string())
            }
        }
        Err(e) => FnOutput::error("serverLockdown", e),
    }
}
