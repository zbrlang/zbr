use crate::context::{DiscordContext, FnOutput};

/// ZinputValue{fieldID} — reads a submitted modal text input field value.
/// Only valid inside an onInteraction handler for a modal submission.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let field_id = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("inputValue", "fieldID is required"),
    };
    FnOutput::Text(ctx.modal_values.get(&field_id).cloned().unwrap_or_default())
}
