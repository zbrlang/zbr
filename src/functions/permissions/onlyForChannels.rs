use crate::context::{DiscordContext, FnOutput};

/// ZonlyForChannels{channelID1;channelID2;...;(errorMessage)}
/// Halts unless the current channel is in the provided list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForChannels", "at least one channel ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("onlyForChannels", "at least one channel ID is required");
    }

    if ids.iter().any(|id| id == &ctx.channel_id) {
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
    (args, "This command can only be used in specific channels.".to_string())
}
