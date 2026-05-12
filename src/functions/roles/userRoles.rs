use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let return_type = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "name".to_string(),
    };
    let separator = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "\n".to_string(),
    };

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userRoles", "invalid user ID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userRoles", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userRoles", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let guild_roles = GuildId::new(gid)
                .roles(&http)
                .await
                .map_err(|e| e.to_string())?;
            let member = GuildId::new(gid)
                .member(&http, UserId::new(uid))
                .await
                .map_err(|e| e.to_string())?;

            let mut list = Vec::new();
            for rid in member.roles {
                if let Some(role) = guild_roles.get(&rid) {
                    list.push(role.clone());
                }
            }

            list.sort_by_key(|r| r.position);
            list.reverse();

            let strings: Vec<String> = if return_type == "name" {
                list.into_iter().map(|r| r.name).collect()
            } else if return_type == "id" {
                list.into_iter().map(|r| r.id.to_string()).collect()
            } else if return_type == "both" {
                list.into_iter()
                    .map(|r| format!("{}:{}", r.name, r.id))
                    .collect()
            } else {
                return Err("invalid returnType".to_string());
            };
            Ok::<String, String>(strings.join(&separator))
        })
    });

    match result {
        Ok(res) => FnOutput::Text(res),
        Err(e) => FnOutput::error("userRoles", e),
    }
}
