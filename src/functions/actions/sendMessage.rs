use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::{validate_bool, validate_snowflake};
use serenity::builder::CreateMessage;
use serenity::model::id::ChannelId;

// args: channel_id ; content ; return_id (optional "true"/"false", default false)
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("sendMessage", "no HTTP client available"),
    };

    let channel_id_str = args.get(0).cloned().unwrap_or_default();
    let channel_id = match validate_snowflake(&channel_id_str, "sendMessage", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let content = args.get(1).cloned().unwrap_or_default();
    if content.is_empty() {
        return FnOutput::error("sendMessage", "message content cannot be empty");
    }

    let return_id = match args.get(2) {
        Some(s) => match validate_bool(s, "sendMessage") {
            Ok(b) => b, Err(e) => return e,
        },
        None => false,
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let msg = CreateMessage::new().content(content);
            match ChannelId::new(channel_id).send_message(&http, msg).await {
                Ok(m) => Ok(if return_id { m.id.to_string() } else { String::new() }),
                Err(e) => Err(format!("sendMessage error: {}", e)),
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(id) => if id.is_empty() { FnOutput::Empty } else { FnOutput::Text(id) },
    }
}
