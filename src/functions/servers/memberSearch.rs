use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let query = match args.get(0) {
        Some(q) if !q.is_empty() => q.clone(),
        _ => return FnOutput::error("memberSearch", crate::error_messages::required(1, "query")),
    };

    let limit: u64 = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(l) => l,
            Err(_) => return FnOutput::error("memberSearch", crate::error_messages::expected_number(2, "limit", s)),
        },
        _ => 100,
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("memberSearch", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("memberSearch", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).search_members(&http, &query, Some(limit)).await
        })
    });

    match result {
        Ok(members) => {
            let ids: Vec<String> = members.iter().map(|m| m.user.id.to_string()).collect();
            FnOutput::Text(ids.join(","))
        }
        Err(e) => FnOutput::error("memberSearch", crate::error_messages::action_failed_reason("search members", &e.to_string())),
    }
}


