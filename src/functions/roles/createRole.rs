use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;
use serenity::builder::EditRole;


pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("createRole", "name is required");
    }

    let color_str = args.get(1).cloned().unwrap_or_default();
    let hoisted_str = args.get(2).cloned().unwrap_or_else(|| "false".to_string());
    let mentionable_str = args.get(3).cloned().unwrap_or_else(|| "false".to_string());

    let mut builder = EditRole::new().name(name);

    if !color_str.is_empty() && color_str != "#000000" {
        let hex = color_str.trim_start_matches('#');
        let c = match u32::from_str_radix(hex, 16) {
            Ok(v) => v,
            Err(_) => return FnOutput::error("createRole", format!("invalid hex color: '{}'", color_str)),
        };
        builder = builder.colour(c as u32);
    }

    if hoisted_str != "false" {
        if hoisted_str == "true" {
            builder = builder.hoist(true);
        } else {
            return FnOutput::error("createRole", format!("invalid boolean for hoisted: '{}'", hoisted_str));
        }
    }

    if mentionable_str != "false" {
        if mentionable_str == "true" {
            builder = builder.mentionable(true);
        } else {
            return FnOutput::error("createRole", format!("invalid boolean for mentionable: '{}'", mentionable_str));
        }
    }

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("createRole", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createRole", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).create_role(&http, builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("createRole", "failed to create role"),
    }
}
