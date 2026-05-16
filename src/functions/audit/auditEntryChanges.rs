use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;
use serde_json::to_string;

/// ZauditEntryChanges{guildID?;entrySelector?} — returns the change details for the selected audit entry as JSON.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditEntryChanges") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let http = match helpers::http_client(ctx, "auditEntryChanges") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(20), "auditEntryChanges") {
        Ok(logs) => logs,
        Err(err) => return err,
    };
    let entry_selector = args.get(1);
    let entry = match helpers::select_entry(&logs.entries, entry_selector, "auditEntryChanges") {
        Ok(entry) => entry,
        Err(err) => return err,
    };

    let changes = entry.changes.clone().unwrap_or_default();
    match to_string(&changes) {
        Ok(json) => FnOutput::Text(json),
        Err(err) => FnOutput::error("auditEntryChanges", err.to_string()),
    }
}
