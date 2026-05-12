use crate::context::{DiscordContext, FnOutput};

/// ZblackListServers{guildID1;guildID2;...;(errorMessage)}
/// Halts if the current guild is in the provided list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("blackListServers", "at least one server ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);

    if ids.iter().any(|id| id == &ctx.guild_id) {
        FnOutput::user_error(error_msg)
    } else {
        FnOutput::Empty
    }
}

fn split_ids_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if last.parse::<u64>().is_err() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "This command is not available in this server.".to_string())
}
