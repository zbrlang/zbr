use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId};

/// ZvoiceBitrate{channelID?}
/// Returns the bitrate (in kbps) of a voice channel. Defaults to the author's current voice channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceBitrate", crate::error_messages::not_in_guild()),
    };

    let channel_id: u64 = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(id) => id,
            Err(_) => {
                return FnOutput::error("voiceBitrate", crate::error_messages::expected_snowflake(1, "channelID", s))
            }
        },
        _ => {
            let author_uid: u64 = match ctx.author_id.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("voiceBitrate", crate::error_messages::expected_snowflake(1, "authorID", &ctx.author_id)),
            };
            match ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
                g.voice_states
                    .get(&serenity::model::id::UserId::new(author_uid))
                    .and_then(|vs| vs.channel_id)
            }) {
                Some(cid) => cid.get(),
                None => {
                    return FnOutput::error(
                        "voiceBitrate",
                        crate::error_messages::required(1, "channelID"),
                    )
                }
            }
        }
    };

    let bitrate = ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
        g.channels
            .get(&ChannelId::new(channel_id))
            .and_then(|ch| ch.bitrate)
    });

    match bitrate {
        Some(bps) => FnOutput::Text((bps / 1000).to_string()),
        None => FnOutput::error("voiceBitrate", crate::error_messages::not_found("channel", &channel_id.to_string())),
    }
}
