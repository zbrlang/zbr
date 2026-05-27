use crate::context::{DiscordContext, FnOutput};
use serenity::model::guild::PremiumTier;
use serenity::model::id::GuildId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if ctx.guild_id.is_empty() {
        return FnOutput::Text("0".to_string());
    }

    let guild_id = match ctx.guild_id.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("boostLevel", crate::error_messages::not_found("guild", &ctx.guild_id)),
    };

    let http = match ctx.http.as_ref() {
        Some(h) => h.clone(),
        None => return FnOutput::error("boostLevel", "no HTTP client available"),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_guild(guild_id).await {
                Ok(guild) => {
                    let level = match guild.premium_tier {
                        PremiumTier::Tier0 => "0",
                        PremiumTier::Tier1 => "1",
                        PremiumTier::Tier2 => "2",
                        PremiumTier::Tier3 => "3",
                        _ => "0",
                    };
                    FnOutput::Text(level.to_string())
                }
                Err(_) => FnOutput::error("boostLevel", crate::error_messages::not_found("guild", &ctx.guild_id)),
            }
        })
    })
}
