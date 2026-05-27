use crate::context::{DiscordContext, FnOutput};
use crate::functions::cooldown::helpers::parse_duration;
use serenity::builder::CreateMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZreplyIn{duration;content}
///
/// Spawns an in-memory task that sleeps, then replies to the trigger message.
/// NOTE: the task is cancelled if the bot restarts.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let dur_str = args.get(0).cloned().unwrap_or_default();
    if dur_str.is_empty() {
        return FnOutput::error("replyIn", crate::error_messages::required(1, "duration"));
    }

    let content = args.get(1).cloned().unwrap_or_default();
    if content.is_empty() {
        return FnOutput::error("replyIn", crate::error_messages::required(2, "content"));
    }

    let secs = match parse_duration(&dur_str) {
        Ok(s) => s,
        Err(e) => {
            if e.contains("must be greater than zero") || dur_str.trim_start().starts_with('-') {
                return FnOutput::error("replyIn", "duration must be at least 1 second");
            }
            return FnOutput::error("replyIn", crate::error_messages::expected_duration(1, "duration", &dur_str));
        }
    } as u64;

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("replyIn", "no HTTP client available"),
    };

    let trigger_id = match &ctx.trigger_message_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => return FnOutput::error("replyIn", "no trigger message available"),
    };

    let channel_id_str = ctx.channel_id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(secs)).await;

        let cid = match channel_id_str.parse::<u64>().ok() {
            Some(id) => ChannelId::new(id),
            None => return,
        };
        let mid = match trigger_id.parse::<u64>().ok() {
            Some(id) => MessageId::new(id),
            None => return,
        };

        if let Ok(trigger_msg) = cid.message(&http, mid).await {
            let msg = CreateMessage::new()
                .content(content)
                .reference_message(&trigger_msg);
            cid.send_message(&http, msg).await.ok();
        }
    });

    FnOutput::Empty
}
