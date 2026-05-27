use crate::context::{DiscordContext, FnOutput};
use serenity::model::guild::VerificationLevel;
use serenity::model::id::GuildId;

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if ctx.guild_id.is_empty() {
        return FnOutput::error("serverVerificationLevel", crate::error_messages::not_in_guild());
    }

    let guild_id = match ctx.guild_id.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::error("serverVerificationLevel", crate::error_messages::not_found("guild", &ctx.guild_id)),
    };

    let http = match ctx.http.as_ref() {
        Some(h) => h.clone(),
        None => return FnOutput::error("serverVerificationLevel", crate::error_messages::action_failed("get HTTP client")),
    };
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_guild(guild_id).await {
                Ok(guild) => {
                    let level = match guild.verification_level {
                        VerificationLevel::None => "none",
                        VerificationLevel::Low => "low",
                        VerificationLevel::Medium => "medium",
                        VerificationLevel::High => "high",
                        VerificationLevel::Higher => "very_high",
                        _ => "none",
                    };
                    FnOutput::Text(level.to_string())
                }
                Err(_) => FnOutput::error("serverVerificationLevel", crate::error_messages::not_found("guild", &ctx.guild_id)),
            }
        })
    })
}
