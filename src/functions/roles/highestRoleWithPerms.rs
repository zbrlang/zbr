use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use crate::functions::permissions::helpers::parse_permission;
use serenity::model::permissions::Permissions;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("highestRoleWithPerms", crate::error_messages::too_few_args(1, args.len()));
    }

    let mut required = Permissions::empty();
    for p in &args {
        match parse_permission(p) {
            Some(perm) => required |= perm,
            None => return FnOutput::error("highestRoleWithPerms", crate::error_messages::unknown_permission(p)),
        }
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("highestRoleWithPerms", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("highestRoleWithPerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).roles(&http).await
        })
    });

    match result {
        Ok(roles) => {
            let highest = roles.values()
                .filter(|r| r.permissions.contains(required) || r.permissions.administrator())
                .max_by_key(|r| r.position);
            
            FnOutput::Text(highest.map(|r| r.id.to_string()).unwrap_or_default())
        }
        Err(_) => FnOutput::error("highestRoleWithPerms", crate::error_messages::action_failed("fetch roles")),
    }
}
