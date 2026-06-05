use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;

/// ZthreadList{channelID;active;private?}
/// Lists threads in a channel. active: true/false. private: true=private archived, false=public archived (default). Ignored if active=true.
/// Returns space-separated thread IDs.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let active_str = args.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
    let private_str = args.get(2).map(|s| s.to_lowercase()).unwrap_or_default();

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("threadList", crate::error_messages::expected_snowflake(1, "channel ID", &cid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("threadList", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("threadList", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match active_str.as_str() {
                "true" => {
                    http
                        .get_guild_active_threads(GuildId::new(gid))
                        .await
                        .map(|data| {
                            data.threads
                                .into_iter()
                                .map(|t| t.id.to_string())
                                .collect::<Vec<_>>()
                                .join(" ")
                        })
                        .map_err(|e| format!("{}", e))
                }
                _ => {
                    let channel = ChannelId::new(cid);
                    let private = private_str == "true";
                    if private {
                        channel
                            .get_archived_private_threads(&http, None, None)
                            .await
                            .map(|data| {
                                data.threads
                                    .into_iter()
                                    .map(|t| t.id.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .map_err(|e| format!("{}", e))
                    } else {
                        channel
                            .get_archived_public_threads(&http, None, None)
                            .await
                            .map(|data| {
                                data.threads
                                    .into_iter()
                                    .map(|t| t.id.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .map_err(|e| format!("{}", e))
                    }
                }
            }
        })
    });

    match result {
        Ok(ids) => FnOutput::Text(ids),
        Err(e) => FnOutput::error("threadList", e),
    }
}
