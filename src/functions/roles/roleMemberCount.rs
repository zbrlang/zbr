use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

/// ZroleMemberCount{roleID?}
/// Defaults to the author's highest role when no roleID is given.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("roleMemberCount", crate::error_messages::not_in_guild()),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("roleMemberCount", "no HTTP client available"),
    };

    let role_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => String::new(),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            // Resolve roleID — default to author's highest role
            let rid: u64 = if role_id_str.is_empty() {
                let author_uid: u64 = ctx
                    .author_id
                    .parse()
                    .map_err(|_| "invalid author ID".to_string())?;
                let roles = GuildId::new(gid)
                    .roles(&http)
                    .await
                    .map_err(|e| e.to_string())?;
                let member = GuildId::new(gid)
                    .member(&http, UserId::new(author_uid))
                    .await
                    .map_err(|_| "author not found".to_string())?;
                member
                    .roles
                    .iter()
                    .filter_map(|rid| roles.get(rid))
                    .max_by_key(|r| r.position)
                    .map(|r| r.id.get())
                    .ok_or_else(|| "author has no roles".to_string())?
            } else {
                role_id_str
                    .parse::<u64>()
                    .map_err(|_| format!("invalid role ID: '{}'", role_id_str))?
            };

            let mut count = 0usize;
            let mut last_id = None::<u64>;
            loop {
                let batch = GuildId::new(gid)
                    .members(&http, Some(1000), last_id.map(UserId::new))
                    .await
                    .map_err(|e| format!("failed to fetch members: {}", e))?;
                if batch.is_empty() {
                    break;
                }
                last_id = Some(batch.last().unwrap().user.id.get());
                count += batch
                    .iter()
                    .filter(|m| m.roles.contains(&RoleId::new(rid)))
                    .count();
                if batch.len() < 1000 {
                    break;
                }
            }
            Ok::<usize, String>(count)
        })
    });

    match result {
        Ok(n) => FnOutput::Text(n.to_string()),
        Err(e) => FnOutput::error("roleMemberCount", e),
    }
}
