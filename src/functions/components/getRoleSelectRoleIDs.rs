use crate::context::{DiscordContext, FnOutput};

/// ZgetRoleSelectRoleIDs{separator;limit?} — returns all selected role IDs joined by separator.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).cloned().unwrap_or_else(|| ", ".to_string());
    let limit: Option<usize> = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(0) | Err(_) => None,
            Ok(n) => Some(n),
        },
        _ => None,
    };
    let values: Vec<&String> = match limit {
        Some(n) => ctx.selected_values.iter().take(n).collect(),
        None => ctx.selected_values.iter().collect(),
    };
    FnOutput::Text(values.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(&separator))
}
