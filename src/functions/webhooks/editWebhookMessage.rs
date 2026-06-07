use crate::context::{DiscordContext, FnOutput};
use serenity::builder::EditWebhookMessage;
use serenity::model::id::{WebhookId, MessageId};

/// ZeditWebhookMessage{webhookID;webhookToken;messageID;content}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let webhook_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("editWebhookMessage", crate::error_messages::required(1, "webhookID")),
    };
    let token = match args.get(1) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("editWebhookMessage", crate::error_messages::required(2, "webhookToken")),
    };
    let mid_str = match args.get(2) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("editWebhookMessage", crate::error_messages::required(3, "messageID")),
    };
    let content = match args.get(3) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("editWebhookMessage", crate::error_messages::required(4, "content")),
    };

    let webhook_id: u64 = match webhook_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editWebhookMessage", crate::error_messages::expected_snowflake(1, "webhookID", webhook_id_str)),
    };
    let mid: u64 = match mid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("editWebhookMessage", crate::error_messages::expected_snowflake(3, "messageID", mid_str)),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editWebhookMessage", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.edit_webhook_message(WebhookId::new(webhook_id), None, token, MessageId::new(mid), &serde_json::json!({"content": content}), vec![]).await
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editWebhookMessage", format!("failed to edit webhook message: {}", e)),
    }
}
