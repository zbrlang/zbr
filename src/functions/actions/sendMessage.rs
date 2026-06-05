use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::{
    validate_bool, validate_channel_same_guild, validate_snowflake,
};
use serenity::builder::CreateMessage;
use serenity::model::id::ChannelId;

// args: channel_id ; content ; return_id (optional "true"/"false", default false)
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("sendMessage", "no HTTP client available"),
    };

    let channel_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or(ctx.channel_id.clone());
    let content = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let return_id_arg = args.get(2).filter(|s| !s.is_empty());

    let channel_id = match validate_snowflake(&channel_id_str, "sendMessage", "channel ID") {
        Ok(id) => id,
        Err(e) => return e,
    };

    if let Err(e) = validate_channel_same_guild(channel_id, ctx, http.clone(), "sendMessage") {
        return e;
    }

    if content.is_empty() {
        return FnOutput::error("sendMessage", crate::error_messages::required(1, "content"));
    }

    let return_id = match return_id_arg {
        Some(s) => match validate_bool(s, "sendMessage") {
            Ok(b) => b,
            Err(e) => return e,
        },
        None => false,
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let msg = CreateMessage::new().content(content);
            match ChannelId::new(channel_id).send_message(&http, msg).await {
                Ok(m) => Ok(if return_id {
                    m.id.to_string()
                } else {
                    String::new()
                }),
                Err(e) => Err(format!("sendMessage error: {}", e)),
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(id) => {
            if id.is_empty() {
                FnOutput::Empty
            } else {
                FnOutput::Text(id)
            }
        }
    }
}
