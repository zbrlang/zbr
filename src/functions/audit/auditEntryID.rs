use crate::context::{DiscordContext, FnOutput};
use crate::functions::audit::helpers;

/// ZauditEntryID{guildID?;entrySelector?} — returns the selected entry's audit log ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id = match helpers::parse_guild_id(&args, ctx, "auditEntryID") {
        Ok(id) => id,
        Err(err) => return err,
    };
    let http = match helpers::http_client(ctx, "auditEntryID") {
        Ok(http) => http,
        Err(err) => return err,
    };
    let logs = match helpers::fetch_audit_logs(guild_id, http, Some(20), "auditEntryID") {
        Ok(logs) => logs,
        Err(err) => return err,
    };
    let entry_selector = args.get(1);
    let entry = match helpers::select_entry(&logs.entries, entry_selector, "auditEntryID") {
        Ok(entry) => entry,
        Err(err) => return err,
    };

    FnOutput::Text(entry.id.to_string())
}
