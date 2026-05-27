use crate::context::{DiscordContext, FnOutput};

/// ZchannelCreated{channelID?}
/// Returns the channel's creation timestamp (Unix seconds) extracted from its snowflake ID.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.channel_id.clone(),
    };
    let id: u64 = match id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "channelCreated",
                crate::error_messages::expected_snowflake(1, "channel ID", &id_str),
            )
        }
    };
    // Discord snowflake: top 42 bits are ms since Discord epoch (2015-01-01)
    let discord_epoch: u64 = 1420070400000;
    let ms = (id >> 22) + discord_epoch;
    let secs = ms / 1000;
    FnOutput::Text(secs.to_string())
}
