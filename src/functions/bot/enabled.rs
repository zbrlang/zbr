use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let condition = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let error_msg = args.get(1).cloned();

    if condition == "true" {
        return FnOutput::Empty;
    } else if condition == "false" {
        if let Some(msg) = error_msg {
            return FnOutput::UserError(msg);
        } else {
            return FnOutput::UserError("This command is currently disabled.".to_string());
        }
    } else {
        return FnOutput::error("enabled", crate::error_messages::expected_boolean(1, "condition", &condition));
    }
}
