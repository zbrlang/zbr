use crate::context::{DiscordContext, FnOutput};

/// ZappEmojis{}
/// Lists the bot application's emojis as a JSON array.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if ctx.bot_id.is_empty() {
        return FnOutput::error("appEmojis", crate::error_messages::requires_set_first("bot ID"));
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("appEmojis", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_application_emojis()
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(emojis) => FnOutput::Text(serde_json::to_string(&emojis).unwrap_or_default()),
        Err(e) => FnOutput::error("appEmojis", e),
    }
}
