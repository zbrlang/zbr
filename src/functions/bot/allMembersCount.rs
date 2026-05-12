use crate::context::{DiscordContext, FnOutput};

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    // Sum member counts across all cached guilds
    let total: u64 = ctx
        .cache
        .guilds()
        .iter()
        .filter_map(|gid| ctx.cache.guild(*gid))
        .map(|g| g.member_count as u64)
        .sum();
    FnOutput::Text(total.to_string())
}
