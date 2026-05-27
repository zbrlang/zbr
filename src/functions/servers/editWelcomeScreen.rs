use crate::context::{DiscordContext, FnOutput};
use serenity::builder::{Builder, CreateGuildWelcomeChannel, EditGuildWelcomeScreen};
use serenity::model::guild::GuildWelcomeChannelEmoji;
use serenity::model::id::{ChannelId, GuildId, EmojiId};

/// ZeditWelcomeScreen{guildID;enabled;description?;welcomeChannelsJSON?}
/// Edits the guild's welcome screen. welcomeChannelsJSON is a JSON array of objects with:
/// {"channel_id":"...","description":"...","emoji_name":"..."} or {"channel_id":"...","description":"...","emoji_id":"...","emoji_name":"..."}
/// welcomeChannelsJSON can be empty string or omitted to clear channels.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let guild_id_str = args.get(0).cloned().unwrap_or_default();
    let enabled_str = args.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
    let description = args.get(2).cloned().unwrap_or_default();
    let channels_json = args.get(3).cloned().unwrap_or_default();

    let guild_id: u64 = match guild_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editWelcomeScreen", crate::error_messages::expected_snowflake(1, "guild ID", &guild_id_str)),
    };

    let enabled = match enabled_str.as_str() {
        "true" => true,
        "false" => false,
        _ => return FnOutput::error("editWelcomeScreen", "enabled must be true or false"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editWelcomeScreen", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditGuildWelcomeScreen::new().enabled(enabled);

            if !description.is_empty() && description != "!unchanged" {
                builder = builder.description(&description);
            }

            if !channels_json.is_empty() && channels_json != "!unchanged" {
                let channels: Vec<serde_json::Value> =
                    serde_json::from_str(&channels_json)
                        .map_err(|e| format!("invalid welcome channels JSON: {}", e))?;

                for ch in channels {
                    let cid_str = ch["channel_id"].as_str().ok_or("channel_id is required")?;
                    let cid: u64 = cid_str.parse().map_err(|_| "invalid channel_id")?;
                    let desc = ch["description"].as_str().unwrap_or("");
                    let mut wc = CreateGuildWelcomeChannel::new(
                        ChannelId::new(cid),
                        desc.to_string(),
                    );
                    if let Some(name) = ch["emoji_name"].as_str() {
                        if let Some(eid_str) = ch["emoji_id"].as_str() {
                            if let Ok(eid) = eid_str.parse::<u64>() {
                                wc = wc.emoji(GuildWelcomeChannelEmoji::Custom {
                                    id: EmojiId::new(eid),
                                    name: name.to_string(),
                                });
                            }
                        } else {
                            wc = wc.emoji(GuildWelcomeChannelEmoji::Unicode(name.to_string()));
                        }
                    }
                    builder = builder.add_welcome_channel(wc);
                }
            }

            builder
                .execute(&http, GuildId::new(guild_id))
                .await
                .map(|_| String::new())
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editWelcomeScreen", e),
    }
}
