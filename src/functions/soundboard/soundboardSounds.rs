use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::GuildId;

/// ZsoundboardSounds{}
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("soundboardSounds", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardSounds", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            GuildId::new(gid).get_soundboards(&http).await
        })
    });

    match result {
        Ok(sounds) => FnOutput::Text(serde_json::to_string(&sounds).unwrap_or_default()),
        Err(e) => FnOutput::error("soundboardSounds", format!("{}", e)),
    }
}
