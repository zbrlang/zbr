use crate::context::{DiscordContext, FnOutput};

/// ZonlyForUsers{username1;username2;...;(errorMessage)}
/// Halts unless the author's username matches one of the provided names.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForUsers", crate::error_messages::too_few_args(1, args.len()));
    }

    let (names, error_msg) = split_last_as_error(&args);
    if names.is_empty() {
        return FnOutput::error("onlyForUsers", crate::error_messages::too_few_args(1, names.len()));
    }

    let username_lower = ctx.username.to_lowercase();
    if names.iter().any(|n| n.to_lowercase() == username_lower) {
        FnOutput::Empty
    } else {
        FnOutput::user_error(error_msg)
    }
}

fn split_last_as_error(args: &[String]) -> (&[String], String) {
    if args.len() > 1 {
        if let Some(last) = args.last() {
            if last.contains(' ') || last.contains('!') || last.contains('.') {
                return (&args[..args.len() - 1], last.clone());
            }
        }
    }
    (args, "You are not allowed to use this command.".to_string())
}
