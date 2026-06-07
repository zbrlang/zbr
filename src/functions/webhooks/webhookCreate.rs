use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::validate_snowflake;
use serenity::builder::CreateWebhook;
use serenity::model::id::ChannelId;

/// ZwebhookCreate{channelID;name}
/// Creates a webhook in the given channel and returns its URL.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let channel_id_str = args.get(0).filter(|s| !s.is_empty()).cloned().unwrap_or_default();
    let name           = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    let channel_id = match validate_snowflake(&channel_id_str, "webhookCreate", "channel ID") {
        Ok(id) => id, Err(e) => return e,
    };
    if name.is_empty() {
        return FnOutput::error("webhookCreate", crate::error_messages::required(2, "name"));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("webhookCreate", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let builder = CreateWebhook::new(name);
            let webhook = ChannelId::new(channel_id)
                .create_webhook(&http, builder).await
                .map_err(|e| format!("failed to create webhook: {}", e))?;
            webhook.url().map_err(|e| format!("failed to get webhook URL: {}", e))
        })
    });

    match result {
        Ok(url) => FnOutput::Text(url),
        Err(e) => FnOutput::error("webhookCreate", e),
    }
}
