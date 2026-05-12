use crate::context::{DiscordContext, FnOutput};
use super::helpers::eval_condition;

/// ZonlyIf{condition;error}
/// Guard function — halts execution with the error message if the condition is false.
/// If the condition is true, does nothing and execution continues.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let cond = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("onlyIf", "condition is required"),
    };
    let error_msg = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("onlyIf", "error message is required"),
    };

    match eval_condition(&cond) {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::UserError(error_msg),
        Err(e) => FnOutput::error("onlyIf", e),
    }
}
