use crate::context::{DiscordContext, FnOutput};

/// ZonlyForServers{guildID1;guildID2;...;(errorMessage)}
/// Halts unless the current guild is in the provided list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForServers", "at least one server ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("onlyForServers", "at least one server ID is required");
    }

    if ids.iter().any(|id| id == &ctx.guild_id) {
        FnOutput::Empty
    } else {
        FnOutput::user_error(error_msg)
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
