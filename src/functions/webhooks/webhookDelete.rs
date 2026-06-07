use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_url;

/// ZwebhookDelete{webhookURL}
/// Deletes a webhook by its URL.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let url = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    if let Err(e) = validate_url(&url, "webhookDelete") { return e; }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("webhookDelete", "no HTTP client available"),
    };

    // Parse webhook ID from URL: https://discord.com/api/webhooks/{id}/{token}
    let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
    let (webhook_id, token) = match (parts.get(parts.len().wrapping_sub(2)), parts.last()) {
        (Some(id), Some(tok)) => (*id, *tok),
        _ => return FnOutput::error("webhookDelete", crate::error_messages::expected_url(1, "webhook URL", &url)),
    };

    let webhook_id: u64 = match webhook_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("webhookDelete", crate::error_messages::expected_snowflake(1, "webhook ID", webhook_id)),
    };

    let token = token.to_string();
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.delete_webhook_with_token(webhook_id.into(), &token, None).await
                .map_err(|e| format!("failed to delete webhook: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("webhookDelete", e),
    }
}
