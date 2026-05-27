use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditMessage;
use serenity::model::id::{ChannelId, MessageId};

/// ZeditMessage{channelID;messageID;content}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let cid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editMessage", crate::error_messages::required(1, "channelID")),
    };
    let mid_str = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editMessage", crate::error_messages::required(2, "messageID")),
    };
    let content = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("editMessage", crate::error_messages::required(3, "content")),
    };

    let cid: u64 = match cid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editMessage", crate::error_messages::expected_snowflake(1, "channelID", &cid_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editMessage", crate::error_messages::expected_snowflake(2, "messageID", &mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editMessage", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            ChannelId::new(cid)
                .edit_message(&http, MessageId::new(mid), EditMessage::new().content(content))
                .await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(_) => FnOutput::error("editMessage", crate::error_messages::action_failed("edit message")),
    }
}
