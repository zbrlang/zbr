use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::{validate_bool, validate_snowflake};
use serenity::builder::CreateThread;
use serenity::model::channel::{AutoArchiveDuration, ChannelType};
use serenity::model::id::{ChannelId, MessageId};

/// ZstartThread{name;channelID;(messageID);(archiveDuration);(returnID);(private)}
/// Creates a thread. If messageID is provided, creates from that message.
/// archiveDuration: 60, 1440, 4320, or 10080 (minutes). Defaults to 60.
/// private: true/false — ignored for message threads. Defaults to private for standalone threads.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = args.get(0).cloned().unwrap_or_default();
    if name.is_empty() {
        return FnOutput::error("startThread", crate::error_messages::required(1, "name"));
    }

    let channel_id_str = args.get(1).cloned().unwrap_or_default();
    let channel_id = match validate_snowflake(&channel_id_str, "startThread", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let message_id_str = args.get(2).cloned().unwrap_or_default();
    let message_id: Option<u64> = if message_id_str.is_empty() || message_id_str == "!unchanged" {
        None
    } else {
        match validate_snowflake(&message_id_str, "startThread", "message ID") {
            Ok(id) => Some(id),
            Err(e) => return e,
        }
    };

    let archive_duration = match args.get(3).map(|s| s.as_str()).unwrap_or("60") {
        "1440"  => AutoArchiveDuration::OneDay,
        "4320"  => AutoArchiveDuration::ThreeDays,
        "10080" => AutoArchiveDuration::OneWeek,
        _       => AutoArchiveDuration::OneHour,
    };

    let return_id = match args.get(4) {
        Some(s) if s != "!unchanged" && !s.is_empty() => match validate_bool(s, "startThread") {
            Ok(b) => b, Err(e) => return e,
        },
        _ => false,
    };

    let is_private = args.get(5).map(|s| s.as_str()).unwrap_or("!unchanged");
    let builder = {
        let mut b = CreateThread::new(&name).auto_archive_duration(archive_duration);
        match is_private {
            "true"  => b = b.kind(ChannelType::PrivateThread),
            "false" => b = b.kind(ChannelType::PublicThread),
            _ => {},
        }
        b
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("startThread", "no HTTP client available"),
    };

    let result: Result<String, String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let thread = if let Some(msg_id) = message_id {
                ChannelId::new(channel_id)
                    .create_thread_from_message(&http, MessageId::new(msg_id), builder).await
                    .map_err(|e| crate::error_messages::action_failed_reason("create thread", &e.to_string()))?
            } else {
                ChannelId::new(channel_id)
                    .create_thread(&http, builder).await
                    .map_err(|e| crate::error_messages::action_failed_reason("create thread", &e.to_string()))?
            };

            Ok(thread.id.to_string())
        })
    });

    match result {
        Ok(id) => if return_id { FnOutput::Text(id) } else { FnOutput::Empty },
        Err(e) => FnOutput::error("startThread", e),
    }
}
