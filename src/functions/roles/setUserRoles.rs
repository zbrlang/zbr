use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};
use serenity::builder::EditMember;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("setUserRoles", "userID and at least one roleID are required");
    }

    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    if uid_str.is_empty() {
        return FnOutput::error("setUserRoles", crate::error_messages::required(1, "user ID"));
    }
    if args.len() < 2 {
        return FnOutput::error("setUserRoles", "at least one roleID is required");
    }
    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("setUserRoles", crate::error_messages::expected_snowflake(1, "user ID", &uid_str)),
    };

    let mut role_ids = Vec::new();
    for arg in &args[1..] {
        let rid: u64 = match arg.parse() {
            Ok(id) => id,
            Err(_) => return FnOutput::error("setUserRoles", format!("invalid role ID: '{}'", arg)),
        };
        role_ids.push(RoleId::new(rid));
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("setUserRoles", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("setUserRoles", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditMember::new().roles(role_ids);
            GuildId::new(gid).edit_member(&http, UserId::new(uid), builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("setUserRoles", crate::error_messages::not_found("user", &uid_str)),
    }
}
