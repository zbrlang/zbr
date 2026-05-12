use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::builder::EditChannel;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use crate::functions::permissions::helpers::parse_permission;
use serenity::model::permissions::Permissions;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.len() < 3 {
        return FnOutput::error("editChannelPerms", "channelID, userOrRoleID, and at least one permission are required");
    }

    let cid_str = args[0].clone();
    let target_str = args[1].clone();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editChannelPerms", format!("invalid channel ID: '{}'", cid_str)),
    };

    let target_id: u64 = match target_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editChannelPerms", format!("invalid user or role ID: '{}'", target_str)),
    };

    let mut allow = Permissions::empty();
    let mut deny = Permissions::empty();

    for arg in &args[2..] {
        if arg.len() < 2 {
            return FnOutput::error("editChannelPerms", format!("permission '{}' must be prefixed with +, -, or /", arg));
        }

        let prefix = &arg[0..1];
        let perm_name = &arg[1..];

        if prefix != "+" && prefix != "-" && prefix != "/" {
            return FnOutput::error("editChannelPerms", format!("permission '{}' must be prefixed with +, -, or /", arg));
        }

        if prefix == "/" {
            continue; // Not explicitly modifying allow/deny adds it to neutral
        }

        let perm = match parse_permission(perm_name) {
            Some(p) => p,
            None => return FnOutput::error("editChannelPerms", format!("unknown permission: '{}'", perm_name)),
        };

        if prefix == "+" {
            allow |= perm;
        } else if prefix == "-" {
            deny |= perm;
        }
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editChannelPerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(cid).to_channel(&http).await.map_err(|_| "channel not found".to_string())?;
            let guild_channel = channel.guild().ok_or_else(|| "not a guild channel".to_string())?;

            let mut is_role = false;
            // Best effort check if it's a role or member. We can just check roles in the guild.
            // Actually, we can just try to fetch the role, or assume role if it's in the guild's roles.
            let roles = guild_channel.guild_id.roles(&http).await.unwrap_or_default();
            if roles.contains_key(&RoleId::new(target_id)) {
                is_role = true;
            }

            let kind = if is_role {
                PermissionOverwriteType::Role(RoleId::new(target_id))
            } else {
                PermissionOverwriteType::Member(UserId::new(target_id))
            };

            let overwrite = PermissionOverwrite { allow, deny, kind };
            
            // To properly implement '/', we need to remove existing overwrites that are 'neutralized',
            // but setting the new overwrite just replaces the existing one for this user/role,
            // so sending just the new allow/deny effectively neutrals everything else!
            ChannelId::new(cid).create_permission(&http, overwrite).await.map_err(|e| e.to_string())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editChannelPerms", e),
    }
}
