use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;
use serde_json::to_string;

/// ZauditEntries{guildID?;limit?} — returns the latest audit entries as JSON.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditEntries") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let limit = helpers::parse_limit(&args, 1);
    let http = match helpers::http_client(ctx, "auditEntries") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(limit), "auditEntries") {
        Ok(logs) => logs,
        Err(err) => return err,
    };

    match to_string(&logs.entries.iter().map(helpers::entry_summary).collect::<Vec<_>>()) {
        Ok(json) => FnOutput::Text(json),
        Err(err) => FnOutput::error("auditEntries", err.to_string()),
    }
}
