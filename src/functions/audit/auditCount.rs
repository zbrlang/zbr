use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;

/// ZauditCount{guildID?;limit?} — returns the number of fetched audit log entries.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditCount") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let limit = helpers::parse_limit(&args, 1);
    let http = match helpers::http_client(ctx, "auditCount") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(limit), "auditCount") {
        Ok(logs) => logs,
        Err(err) => return err,
    };

    FnOutput::Text(logs.entries.len().to_string())
}
