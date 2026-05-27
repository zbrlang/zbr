use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let separator = args.get(0).cloned().unwrap_or_else(|| "\n".to_string());

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("roleNames", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("roleNames", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).roles(&http).await
        })
    });

    match result {
        Ok(roles) => {
            let mut list: Vec<_> = roles.values().collect();
            list.sort_by_key(|r| r.position);
            list.reverse(); // Highest first usually
            let names: Vec<String> = list.into_iter().map(|r| r.name.clone()).collect();
            FnOutput::Text(names.join(&separator))
        }
        Err(_) => FnOutput::error("roleNames", crate::error_messages::action_failed("fetch roles")),
    }
}
