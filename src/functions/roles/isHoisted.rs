use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let role_id_str = if args.get(0).map(|s| s.is_empty()).unwrap_or(true) {
        // fetch author's top role
        let http = ctx.http.as_ref().unwrap().clone();
        let guild_id = ctx.guild_id.parse::<u64>().map(GuildId::new).unwrap();
        let user_id = ctx.author_id.parse::<u64>().map(UserId::new).unwrap();
        let member = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async move { http.get_member(guild_id, user_id).await })
        });
        match member {
            Ok(m) => m.roles.first().map(|r| r.to_string()).unwrap_or_default(),
            Err(_) => return FnOutput::error("isHoisted", "could not get author's top role"),
        }
    } else {
        args[0].clone()
    };

    let rid: u64 = match role_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isHoisted", "invalid role ID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isHoisted", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isHoisted", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(gid)
                .roles(&http)
                .await
                .map_err(|e| e.to_string())?;
            if let Some(role) = roles.get(&RoleId::new(rid)) {
                Ok(role.hoist)
            } else {
                Err("role not found".to_string())
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(if val {
            "true".to_string()
        } else {
            "false".to_string()
        }),
        Err(e) => FnOutput::error("isHoisted", e),
    }
}
