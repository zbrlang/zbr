use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{ChannelId, GuildId, UserId};

/// ZvoiceMembers{channelID?}
/// Defaults to the author's current voice channel if no channelID is given.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("voiceMembers", crate::error_messages::not_in_guild()),
    };

    let channel_id: u64 = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse() {
            Ok(id) => id,
            Err(_) => {
                return FnOutput::error("voiceMembers", crate::error_messages::expected_snowflake(1, "channelID", s))
            }
        },
        _ => {
            // Default to the author's current voice channel
            let author_uid: u64 = match ctx.author_id.parse() {
                Ok(id) => id,
                Err(_) => return FnOutput::error("voiceMembers", crate::error_messages::expected_snowflake(1, "authorID", &ctx.author_id)),
            };
            match ctx.cache.guild(GuildId::new(gid)).and_then(|g| {
                g.voice_states
                    .get(&UserId::new(author_uid))
                    .and_then(|vs| vs.channel_id)
            }) {
                Some(cid) => cid.get(),
                None => {
                    return FnOutput::error(
                        "voiceMembers",
                        crate::error_messages::required(1, "channelID"),
                    )
                }
            }
        }
    };

    let target = ChannelId::new(channel_id);

    let members = ctx
        .cache
        .guild(GuildId::new(gid))
        .map(|g| {
            g.voice_states
                .iter()
                .filter(|(_, vs)| vs.channel_id == Some(target))
                .map(|(uid, _)| uid.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    FnOutput::Text(members.join(" "))
}
