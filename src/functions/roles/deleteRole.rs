use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let rid_str = args.get(0).cloned().unwrap_or_default();
    if rid_str.is_empty() {
        return FnOutput::error("deleteRole", "role ID is required");
    }

    let rid: u64 = match rid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteRole", format!("invalid role ID: '{}'", rid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("deleteRole", "not in a guild"),
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
        Err(_) => FnOutput::error("deleteRole", "role not found"),
    }
}
