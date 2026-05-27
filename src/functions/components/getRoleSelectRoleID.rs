use crate::context::{DiscordContext, FnOutput};

/// ZgetRoleSelectRoleID{index} — returns the role ID at the given 1-based index from a role select interaction.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            Ok(_) => return FnOutput::error("getRoleSelectRoleID", crate::error_messages::must_be_positive(1, "index", 0)),
            Err(_) => return FnOutput::error("getRoleSelectRoleID", crate::error_messages::expected_integer(1, "index", s)),
        },
        _ => return FnOutput::error("getRoleSelectRoleID", crate::error_messages::required(1, "index")),
    };
    FnOutput::Text(ctx.selected_values.get(index - 1).cloned().unwrap_or_default())
}
