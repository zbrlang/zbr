use crate::context::{DiscordContext, FnOutput};

/// ZignoreChannels{channelID1;channelID2;...;(errorMessage)}
/// Halts if the current channel is in the provided list.
/// Last arg is treated as error message if it doesn't parse as a snowflake.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("ignoreChannels", "at least one channel ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("ignoreChannels", "at least one channel ID is required");
    }

    if ids.iter().any(|id| id == &ctx.channel_id) {
        return FnOutput::user_error(error_msg);
    }

    FnOutput::Empty
}

fn split_ids_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if last.parse::<u64>().is_err() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "This command cannot be used in this channel.".to_string())
}
