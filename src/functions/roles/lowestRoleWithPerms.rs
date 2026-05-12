use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use crate::functions::permissions::helpers::parse_permission;
use serenity::model::permissions::Permissions;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("lowestRoleWithPerms", "at least one permission is required");
    }

    let mut required = Permissions::empty();
    for p in &args {
        match parse_permission(p) {
            Some(perm) => required |= perm,
            None => return FnOutput::error("lowestRoleWithPerms", format!("unknown permission: '{}'", p)),
        }
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("lowestRoleWithPerms", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("lowestRoleWithPerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).roles(&http).await
        })
    });

    match result {
        Ok(roles) => {
            let lowest = roles.values()
                .filter(|r| r.id.get() != gid) // Exclude @everyone
                .filter(|r| r.permissions.contains(required) || r.permissions.administrator())
                .min_by_key(|r| r.position);
            
            FnOutput::Text(lowest.map(|r| r.id.to_string()).unwrap_or_default())
        }
        Err(_) => FnOutput::error("lowestRoleWithPerms", "failed to fetch roles"),
    }
}
