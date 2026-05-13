use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId, UserId};

/// ZvoiceMemberCount{channelID?}
/// Defaults to the author's current voice channel if no channelID is given.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceMemberCount", "not in a guild"),
    };

    let channel_id: u64 = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(id) => id,
            Err(_) => {
                return FnOutput::error("voiceMemberCount", format!("invalid channel ID: '{}'", s))
            }
        },
        _ => {
            let author_uid: u64 = match ctx.author_id.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("voiceMemberCount", "invalid author ID"),
            };
            match ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
                g.voice_states
                    .get(&UserId::new(author_uid))
                    .and_then(|vs| vs.channel_id)
            }) {
                Some(cid) => cid.get(),
                None => {
                    return FnOutput::error(
                        "voiceMemberCount",
                        "channelID is required (author is not in a voice channel)",
                    )
                }
            }
        }
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

    FnOutput::Text(count.to_string())
}
