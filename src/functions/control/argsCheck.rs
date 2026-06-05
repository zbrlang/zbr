use crate::context::{DiscordContext, FnOutput};

/// ZargsCheck{min;max?;error}
/// Halts with error if the number of space-separated args in ctx.message
/// (after the trigger prefix) is outside the allowed range.
/// max is optional — omit or pass empty string for no upper limit.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let min: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(n) => n,
            Err(_) => return FnOutput::error("argsCheck", crate::error_messages::expected_integer(1, "min", s)),
        },
        _ => return FnOutput::error("argsCheck", crate::error_messages::required(1, "min")),
    };

    // If 3 args: min;max;error. If 2 args: min;error (no max).
    let (max, error_msg) = if args.len() >= 3 {
        let max_str = args.get(1).map(|s| s.as_str()).unwrap_or("");
        let max: Option<usize> = if max_str.is_empty() {
            None
        } else {
            match max_str.parse() {
                Ok(n) => Some(n),
                Err(_) => return FnOutput::error("argsCheck", crate::error_messages::expected_integer(2, "max", max_str)),
            }
        };
        let err = args.get(2).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
        (max, err)
    } else {
        let err = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
        (None, err)
    };

    // Count args in the message — strip the trigger prefix first
    let message = ctx.message.trim();
    let body = if let Some(trigger) = &ctx.trigger {
        message.strip_prefix(trigger.as_str()).unwrap_or(message).trim()
    } else {
        message
    };

    let arg_count = if body.is_empty() {
        0
    } else {
        body.split_whitespace().count()
    };

    if arg_count < min {
        return FnOutput::UserError(error_msg);
    }
    if let Some(max) = max {
        if arg_count > max {
            return FnOutput::UserError(error_msg);
        }
    }

    FnOutput::Empty
}
