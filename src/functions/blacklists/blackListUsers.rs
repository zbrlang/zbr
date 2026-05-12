use crate::context::{DiscordContext, FnOutput};

/// ZblackListUsers{username1;username2;...;(errorMessage)}
/// Halts if the author's username matches any in the provided list.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("blackListUsers", "at least one username is required");
    }

    let (names, error_msg) = split_last_as_error(&args);

    let username_lower = ctx.username.to_lowercase();
    if names.iter().any(|n| n.to_lowercase() == username_lower) {
        FnOutput::user_error(error_msg)
    } else {
        FnOutput::Empty
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
    (args, "You are blacklisted from using this command.".to_string())
}
