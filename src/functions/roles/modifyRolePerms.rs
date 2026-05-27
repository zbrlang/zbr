use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId};
use serenity::builder::EditRole;
use crate::functions::permissions::helpers::parse_permission;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 2 {
        return FnOutput::error("modifyRolePerms", crate::error_messages::too_few_args(2, args.len()));
    }

    let rid_str = args[0].clone();
    let rid: u64 = match rid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("modifyRolePerms", crate::error_messages::expected_snowflake(1, "role ID", &rid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("modifyRolePerms", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("modifyRolePerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(gid).roles(&http).await.map_err(|_| crate::error_messages::action_failed("fetch roles"))?;
            let role = roles.get(&RoleId::new(rid)).ok_or_else(|| crate::error_messages::not_found("role", &rid_str))?;

            let mut permissions = role.permissions;

            for arg in &args[1..] {
                if arg.len() < 2 {
                    return Err(format!("permission '{}' must be prefixed with +, -, or /", arg));
                }

                let prefix = &arg[0..1];
                let perm_name = &arg[1..];

                if prefix != "+" && prefix != "-" && prefix != "/" {
                    return Err(format!("permission '{}' must be prefixed with +, -, or /", arg));
                }

                let perm = match parse_permission(perm_name) {
                    Some(p) => p,
                    None => return Err(crate::error_messages::unknown_permission(perm_name)),
                };

                if prefix == "+" {
                    permissions |= perm;
                } else if prefix == "-" {
                    permissions &= !perm;
                } else if prefix == "/" {
                    // / for a role usually means resetting to default.
                    // But Discord roles don't have a "neutral" state like channel overwrites do.
                    // They only have allow or not allow. So '/' usually removes it, like '-'.
                    permissions &= !perm;
                }
            }

            let builder = EditRole::new().permissions(permissions);
            GuildId::new(gid).edit_role(&http, RoleId::new(rid), builder).await.map_err(|_| crate::error_messages::not_found("role", &rid_str))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("modifyRolePerms", e),
    }
}
