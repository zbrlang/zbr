use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;

/// ZauditEntryTarget{guildID?;entrySelector?} — returns the target ID for the selected audit entry.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditEntryTarget") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let http = match helpers::http_client(ctx, "auditEntryTarget") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(20), "auditEntryTarget") {
        Ok(logs) => logs,
        Err(err) => return err,
    };
    let entry_selector = args.get(1);
    let entry = match helpers::select_entry(&logs.entries, entry_selector, "auditEntryTarget") {
        Ok(entry) => entry,
        Err(err) => return err,
    };

    FnOutput::Text(entry.target_id.as_ref().map(|id| id.to_string()).unwrap_or_default())
}
