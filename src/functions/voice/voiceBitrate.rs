use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId};

/// ZvoiceBitrate{channelID?}
/// Returns the bitrate (in kbps) of a voice channel. Defaults to the author's current voice channel.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceBitrate", "not in a guild"),
    };

    let channel_id: u64 = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(id) => id,
            Err(_) => {
                return FnOutput::error("voiceBitrate", format!("invalid channel ID: '{}'", s))
            }
        },
        _ => {
            let author_uid: u64 = match ctx.author_id.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("voiceBitrate", "invalid author ID"),
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
                        "channelID is required (author is not in a voice channel)",
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
        None => FnOutput::error("voiceBitrate", "channel not found or not a voice channel"),
    }
}
