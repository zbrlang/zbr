use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let role_id_str = args.get(1).cloned().unwrap_or_default();

    if role_id_str.is_empty() {
        return FnOutput::error("hasRole", "role ID is required");
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", "invalid user ID"),
    };

    let rid: u64 = match role_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", "invalid role ID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("hasRole", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(gid)
                .member(&http, UserId::new(uid))
                .await
                .map_err(|e| e.to_string())?;
            Ok::<bool, String>(member.roles.contains(&RoleId::new(rid)))
        })
    });

    match result {
        Ok(has) => FnOutput::Text(if has {
            "true".to_string()
        } else {
            "false".to_string()
        }),
        Err(_) => FnOutput::error("hasRole", "user not found"),
    }
}
