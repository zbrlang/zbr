use crate::context::{DiscordContext, FnOutput};

/// ZgetUserSelectUserID{index} — returns the user ID at the given 1-based index from a user select interaction.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            _ => return FnOutput::error("getUserSelectUserID", format!("invalid index: '{}' (must be 1 or greater)", s)),
        },
        _ => return FnOutput::error("getUserSelectUserID", "index is required"),
    };
    FnOutput::Text(ctx.selected_values.get(index - 1).cloned().unwrap_or_default())
}
