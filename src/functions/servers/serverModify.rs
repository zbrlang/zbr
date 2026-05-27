use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditGuild;
use serenity::model::id::GuildId;

/// ZserverModify{guildID?;name?;icon?;banner?;splash?;description?;afkChannelID?;afkTimeout?;systemChannelID?;verificationLevel?;defaultMessageNotifications?;explicitContentFilter?}
/// Edits guild settings. Use !unchanged for fields to skip.
/// icon: URL to download as the new icon, or empty to remove.
/// banner/splash: base64 data URI (data:image/...;base64,...) or empty to remove.
/// verificationLevel: "none", "low", "medium", "high", "very_high"
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| ctx.guild_id.clone());
    if guild_id_str.is_empty() {
        return FnOutput::error("serverModify", crate::error_messages::not_in_guild());
    }

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("serverModify", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverModify", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditGuild::new();

            let name = args.get(1).map(|s| s.as_str()).unwrap_or("!unchanged");
            if name != "!unchanged" && !name.is_empty() {
                builder = builder.name(name);
            }

            let icon = args.get(2).map(|s| s.as_str()).unwrap_or("!unchanged");
            if icon != "!unchanged" {
                if icon.is_empty() {
                    builder = builder.delete_icon();
                } else {
                    let attachment = serenity::builder::CreateAttachment::url(&http, icon)
                        .await
                        .map_err(|e| crate::error_messages::action_failed_reason("download icon", &format!("{}", e)))?;
                    builder = builder.icon(Some(&attachment));
                }
            }

            let banner = args.get(3).map(|s| s.as_str()).unwrap_or("!unchanged");
            if banner != "!unchanged" {
                builder = builder.banner(if banner.is_empty() { None } else { Some(banner.to_string()) });
            }

            let splash = args.get(4).map(|s| s.as_str()).unwrap_or("!unchanged");
            if splash != "!unchanged" {
                builder = builder.splash(if splash.is_empty() { None } else { Some(splash.to_string()) });
            }

            let description = args.get(5).map(|s| s.as_str()).unwrap_or("!unchanged");
            if description != "!unchanged" && !description.is_empty() {
                builder = builder.description(description);
            }

            let afk_cid = args.get(6).map(|s| s.as_str()).unwrap_or("!unchanged");
            if afk_cid != "!unchanged" {
                if afk_cid.is_empty() {
                    builder = builder.afk_channel(None);
                } else {
                    if let Ok(cid) = afk_cid.parse::<u64>() {
                        builder = builder.afk_channel(Some(serenity::model::id::ChannelId::new(cid)));
                    }
                }
            }

            let afk_timeout = args.get(7).map(|s| s.as_str()).unwrap_or("!unchanged");
            if afk_timeout != "!unchanged" && !afk_timeout.is_empty() {
                if let Ok(t) = afk_timeout.parse::<u64>() {
                    builder = builder.afk_timeout(serenity::model::guild::AfkTimeout::from(t as u16));
                }
            }

            let sys_cid = args.get(8).map(|s| s.as_str()).unwrap_or("!unchanged");
            if sys_cid != "!unchanged" {
                if sys_cid.is_empty() {
                    builder = builder.system_channel_id(None);
                } else {
                    if let Ok(cid) = sys_cid.parse::<u64>() {
                        builder = builder.system_channel_id(Some(serenity::model::id::ChannelId::new(cid)));
                    }
                }
            }

            let ver_level = args.get(9).map(|s| s.as_str()).unwrap_or("!unchanged");
            if ver_level != "!unchanged" && !ver_level.is_empty() {
                use serenity::model::guild::VerificationLevel;
                let lvl = match ver_level {
                    "none" => VerificationLevel::None,
                    "low" => VerificationLevel::Low,
                    "medium" => VerificationLevel::Medium,
                    "high" => VerificationLevel::High,
                    "very_high" | "higher" => VerificationLevel::Higher,
                    _ => return Err(crate::error_messages::expected_choice(10, "verificationLevel", "none, low, medium, high, very_high", ver_level)),
                };
                builder = builder.verification_level(lvl);
            }

            GuildId::new(guild_id)
                .edit(&http, builder)
                .await
                .map(|_| String::new())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("serverModify", e),
    }
}
