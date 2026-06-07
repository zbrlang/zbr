use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{WebhookId, MessageId};

/// ZgetWebhookMessage{webhookID;webhookToken;messageID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let webhook_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("getWebhookMessage", crate::error_messages::required(1, "webhookID")),
    };
    let token = match args.get(1) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("getWebhookMessage", crate::error_messages::required(2, "webhookToken")),
    };
    let mid_str = match args.get(2) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("getWebhookMessage", crate::error_messages::required(3, "messageID")),
    };

    let webhook_id: u64 = match webhook_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getWebhookMessage", crate::error_messages::expected_snowflake(1, "webhookID", webhook_id_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("getWebhookMessage", crate::error_messages::expected_snowflake(3, "messageID", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("getWebhookMessage", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_webhook_message(WebhookId::new(webhook_id), None, token, MessageId::new(mid)).await
        })
    });

    match result {
        Ok(msg) => FnOutput::Text(msg.content),
        Err(e) => FnOutput::error("getWebhookMessage", format!("failed to get webhook message: {}", e)),
    }
}
