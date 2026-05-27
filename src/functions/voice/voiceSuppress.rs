use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{Builder, EditVoiceState};
use serenity::model::id::{ChannelId, GuildId, UserId};

/// ZvoiceSuppress{userID;suppress;channelID}
/// Suppress or unsuppress a user in a stage channel. suppress: true/false. channelID is the stage channel ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = args.get(0).cloned().unwrap_or_default();
    let suppress_str = args
        .get(1)
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let cid_str = args.get(2).cloned().unwrap_or_default();

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceSuppress", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let suppress = match suppress_str.as_str() {
        "true" => true,
        "false" => false,
        _ => return FnOutput::error("voiceSuppress", crate::error_messages::expected_boolean(2, "suppress", &suppress_str)),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceSuppress", crate::error_messages::expected_snowflake(3, "channelID", &cid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceSuppress", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceSuppress", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = EditVoiceState::new().suppress(suppress);
            builder
                .execute(&http, (GuildId::new(gid), ChannelId::new(cid), Some(UserId::new(uid))))
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("voiceSuppress", e),
    }
}
