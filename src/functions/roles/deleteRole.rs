use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let rid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if rid_str.is_empty() {
        return FnOutput::error("deleteRole", crate::error_messages::required(1, "role ID"));
    }

    let rid: u64 = match rid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteRole", crate::error_messages::expected_snowflake(1, "role ID", &rid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteRole", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteRole", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).delete_role(&http, RoleId::new(rid)).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("deleteRole", crate::error_messages::not_found("role", &rid_str)),
    }
}
