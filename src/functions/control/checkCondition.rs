use crate::context::{DiscordContext, FnOutput};
use super::helpers::eval_condition;

/// ZcheckCondition{condition}
/// Evaluates a condition string and returns "true" or "false".
/// Same condition syntax as Zif: ==, !=, >, <, >=, <=, contains, startsWith, endsWith, &&, ||
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let cond = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("checkCondition", crate::error_messages::required(1, "condition")),
    };

    match eval_condition(&cond) {
        Ok(result) => FnOutput::Text(result.to_string()),
        Err(e) => FnOutput::error("checkCondition", e),
    }
}
