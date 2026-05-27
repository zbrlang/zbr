use crate::context::{DiscordContext, FnOutput};
use serenity::model::channel::ChannelType;
use serenity::model::id::{ChannelId, GuildId, UserId};

/// ZvoiceFull{channelID?}
/// Defaults to the author's current voice channel if no channelID is given.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceFull", crate::error_messages::not_in_guild()),
    };

    let channel_id: u64 = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(id) => id,
            Err(_) => return FnOutput::error("voiceFull", crate::error_messages::expected_snowflake(1, "channelID", s)),
        },
        _ => {
            let author_uid: u64 = match ctx.author_id.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("voiceFull", crate::error_messages::expected_snowflake(1, "authorID", &ctx.author_id)),
            };
            match ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
                g.voice_states
                    .get(&UserId::new(author_uid))
                    .and_then(|vs| vs.channel_id)
            }) {
                Some(cid) => cid.get(),
                None => {
                    return FnOutput::error(
                        "voiceFull",
                        crate::error_messages::required(1, "channelID"),
                    )
                }
            }
        }
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("voiceFull", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(channel_id).to_channel(&http).await
        })
    });

    let user_limit = match result {
        Ok(channel) => {
            if let Some(guild_channel) = channel.guild() {
                if guild_channel.kind != ChannelType::Voice && guild_channel.kind != ChannelType::Stage {
                    return FnOutput::error("voiceFull", crate::error_messages::action_failed_reason("verify channel type", "not a voice or stage channel"));
                }
                guild_channel.user_limit.unwrap_or(0) as usize
            } else {
                return FnOutput::error("voiceFull", "channel is not a voice or stage channel");
            }
        }
        Err(_) => return FnOutput::error("voiceFull", crate::error_messages::not_found("channel", &channel_id.to_string())),
    };

    let target = ChannelId::new(channel_id);

    let count = ctx
        .cache
        .guild(GuildId::new(gid))
        .map(|g| {
            g.voice_states
                .values()
                .filter(|vs| vs.channel_id == Some(target))
                .count()
        })
        .unwrap_or(0);

    FnOutput::Text((count == user_limit).to_string())
}
