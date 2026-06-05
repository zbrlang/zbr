use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let role_id_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    if role_id_str.is_empty() {
        return FnOutput::error("hasRole", crate::error_messages::required(2, "role ID"));
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", crate::error_messages::expected_snowflake(1, "user ID", &user_id_str)),
    };

    let rid: u64 = match role_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", crate::error_messages::expected_snowflake(2, "role ID", &role_id_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("hasRole", crate::error_messages::not_in_guild()),
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
        Err(_) => FnOutput::error("hasRole", crate::error_messages::not_found("user", &user_id_str)),
    }
}
