use crate::context::{DiscordContext, FnOutput};

/// ZgetMentionableSelectUserID{index} — returns the user/role ID at the given 1-based index from a mentionable select interaction.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            _ => return FnOutput::error("getMentionableSelectUserID", format!("invalid index: '{}' (must be 1 or greater)", s)),
        },
        _ => return FnOutput::error("getMentionableSelectUserID", crate::error_messages::required(1, "index")),
    };
    FnOutput::Text(ctx.selected_values.get(index - 1).cloned().unwrap_or_default())
}
