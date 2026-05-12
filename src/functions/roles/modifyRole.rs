use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId};
use serenity::builder::EditRole;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let rid_str = args.get(0).cloned().unwrap_or_default();
    if rid_str.is_empty() {
        return FnOutput::error("modifyRole", "role ID is required");
    }

    let rid: u64 = match rid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("modifyRole", format!("invalid role ID: '{}'", rid_str)),
    };

    let name = args.get(1).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let color_str = args.get(2).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let hoisted_str = args.get(3).cloned().unwrap_or_else(|| "!unchanged".to_string());
    let mentionable_str = args.get(4).cloned().unwrap_or_else(|| "!unchanged".to_string());

    let mut builder = EditRole::new();

    if name != "!unchanged" {
        builder = builder.name(name);
    }

    if color_str != "!unchanged" {
        let hex = color_str.trim_start_matches('#');
        let c = match u32::from_str_radix(hex, 16) {
            Ok(v) => v,
            Err(_) => return FnOutput::error("modifyRole", format!("invalid hex color: '{}'", color_str)),
        };
        builder = builder.colour(c as u32);
    }

    if hoisted_str != "!unchanged" {
        match hoisted_str.as_str() {
            "true" => builder = builder.hoist(true),
            "false" => builder = builder.hoist(false),
            _ => return FnOutput::error("modifyRole", format!("invalid boolean for hoisted: '{}'", hoisted_str)),
        }
    }

    if mentionable_str != "!unchanged" {
        match mentionable_str.as_str() {
            "true" => builder = builder.mentionable(true),
            "false" => builder = builder.mentionable(false),
            _ => return FnOutput::error("modifyRole", format!("invalid boolean for mentionable: '{}'", mentionable_str)),
        }
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("modifyRole", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("modifyRole", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).edit_role(&http, RoleId::new(rid), builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("modifyRole", "role not found"),
    }
}
