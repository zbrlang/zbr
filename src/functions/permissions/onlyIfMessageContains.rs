use crate::context::{DiscordContext, FnOutput};

/// ZonlyIfMessageContains{message;word;...;error}
/// Halts with error if the message does not contain ALL of the provided words.
/// Last arg is always the error message.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("onlyIfMessageContains", crate::error_messages::too_few_args(3, args.len()));
    }

    let message = args[0].to_lowercase();
    let error_msg = args.last().unwrap().clone();
    // words are everything between the message and the last error arg
    let words = &args[1..args.len() - 1];

    let all_present = words.iter().all(|w| message.contains(&w.to_lowercase()));

    if all_present {
        FnOutput::Empty
    } else {
        FnOutput::UserError(error_msg)
    }
}
