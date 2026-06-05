use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId};
use serenity::builder::EditRole;

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let rid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let color_str = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    if rid_str.is_empty() || color_str.is_empty() {
        return FnOutput::error("colorRole", "role ID and color are required");
    }

    let rid: u64 = match rid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("colorRole", crate::error_messages::expected_snowflake(1, "role ID", &rid_str)),
    };

    let hex = color_str.trim_start_matches('#');
    let c = match u32::from_str_radix(hex, 16) {
        Ok(v) => v,
        Err(_) => return FnOutput::error("colorRole", crate::error_messages::expected_hex_color(2, "color", &color_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("colorRole", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("colorRole", "no HTTP client available"),
    };

    let builder = EditRole::new().colour(c as u32);

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).edit_role(&http, RoleId::new(rid), builder).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("colorRole", crate::error_messages::not_found("role", &rid_str)),
    }
}
