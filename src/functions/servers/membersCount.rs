use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::model::user::OnlineStatus;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if ctx.guild_id.is_empty() {
        return FnOutput::error("membersCount", "not in a guild");
    }

    let guild_id = match ctx.guild_id.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("membersCount", "guild not found"),
    };

    let presence_filter = match args.get(0) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    };

    if let Some(filter) = presence_filter {
        // Presence data is only available in the cache
        let count = ctx
            .cache
            .guild(guild_id)
            .map(|g| {
                g.presences
                    .values()
                    .filter(|p| match filter.as_str() {
                        "online" => p.status == OnlineStatus::Online,
                        "idle" => p.status == OnlineStatus::Idle,
                        "dnd" => p.status == OnlineStatus::DoNotDisturb,
                        "offline" => {
                            p.status == OnlineStatus::Offline
                                || p.status == OnlineStatus::Invisible
                        }
                        _ => false,
                    })
                    .count()
            })
            .unwrap_or(0);
        FnOutput::Text(count.to_string())
    } else {
        // Total member count — use cache member_count if available, else HTTP
        if let Some(guild) = ctx.cache.guild(guild_id) {
            return FnOutput::Text(guild.member_count.to_string());
        }

        let http = match ctx.http.as_ref() {
            Some(h) => h.clone(),
            None => return FnOutput::error("membersCount", "no HTTP client available"),
        };
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                match http.get_guild(guild_id).await {
                    Ok(guild) => {
                        FnOutput::Text(guild.approximate_member_count.unwrap_or(0).to_string())
                    }
                    Err(_) => FnOutput::error("membersCount", "guild not found"),
                }
            })
        })
    }
}
