use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("roleGrant", "userID and at least one role to grant/remove are required");
    }

    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    if uid_str.is_empty() {
        return FnOutput::error("roleGrant", crate::error_messages::required(1, "user ID"));
    }
    if args.len() < 2 {
        return FnOutput::error("roleGrant", "at least one role to grant/remove is required");
    }
    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("roleGrant", crate::error_messages::expected_snowflake(1, "user ID", &uid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("roleGrant", crate::error_messages::not_in_guild()),
    };

    let mut ops = Vec::new(); // (RoleId, is_add)
    for arg in &args[1..] {
        if arg.len() < 2 {
            return FnOutput::error("roleGrant", format!("role '{}' must be prefixed with + or -", arg));
        }

        let prefix = &arg[0..1];
        let rid_str = &arg[1..];

        if prefix != "+" && prefix != "-" {
            return FnOutput::error("roleGrant", format!("role '{}' must be prefixed with + or -", arg));
        }

        let rid: u64 = match rid_str.parse() {
            Ok(id) => id,
            Err(_) => return FnOutput::error("roleGrant", format!("invalid role ID: '{}'", rid_str)),
        };

        ops.push((RoleId::new(rid), prefix == "+"));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("roleGrant", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(gid).member(&http, UserId::new(uid)).await
                .map_err(|_| "user not found".to_string())?;

            for (rid, is_add) in ops {
                if is_add {
                    member.add_role(&http, rid).await.map_err(|_| format!("role not found: '{}'", rid.get()))?;
                } else {
                    member.remove_role(&http, rid).await.map_err(|_| format!("role not found: '{}'", rid.get()))?;
                }
            }

            Ok::<(), String>(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("roleGrant", e),
    }
}
