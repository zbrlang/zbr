use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let amount = args.get(0).and_then(|s| s.parse::<usize>().ok());
    let separator = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "\n".to_string());

    // Collect guild names from cache
    let mut names: Vec<String> = ctx
        .cache
        .guilds()
        .iter()
        .filter_map(|gid| ctx.cache.guild(*gid))
        .map(|g| g.name.clone())
        .collect();

    if let Some(limit) = amount {
        names.truncate(limit);
    }

    if names.is_empty() {
        return FnOutput::Empty;
    }

    FnOutput::Text(names.join(&separator))
}
