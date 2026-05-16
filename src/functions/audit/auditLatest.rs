use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;
use serde_json::to_string;

/// ZauditLatest{guildID?} — returns the latest audit entry as JSON.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditLatest") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let http = match helpers::http_client(ctx, "auditLatest") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(1), "auditLatest") {
        Ok(logs) => logs,
        Err(err) => return err,
    };
    let entry = match logs.entries.first() {
        Some(entry) => entry,
        None => return FnOutput::error("auditLatest", "no audit entries found"),
    };

    match to_string(&helpers::entry_summary(entry)) {
        Ok(json) => FnOutput::Text(json),
        Err(err) => FnOutput::error("auditLatest", err.to_string()),
    }
}
