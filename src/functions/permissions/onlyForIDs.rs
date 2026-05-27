use crate::context::{DiscordContext, FnOutput};

/// ZonlyForIDs{userID1;userID2;...;(errorMessage)}
/// Halts unless the author's ID is in the provided list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForIDs", crate::error_messages::too_few_args(1, args.len()));
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("onlyForIDs", crate::error_messages::too_few_args(1, ids.len()));
    }

    if ids.iter().any(|id| id == &ctx.author_id) {
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
    (args, "You are not allowed to use this command.".to_string())
}
