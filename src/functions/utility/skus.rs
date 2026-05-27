use crate::context::{DiscordContext, FnOutput};

/// Zskus{}
/// Returns the bot application's SKUs as a JSON array.
/// Only useful for premium bots with monetization.
/// Note: Uses raw Discord API since serenity 0.12 doesn't expose this endpoint.
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("skus", crate::error_messages::action_failed("get HTTP client")),
    };

    let token = http.token().to_string();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            client
                .get("https://discord.com/api/v10/applications/@me/skus")
                .header("Authorization", &token)
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?
                .text()
                .await
                .map_err(|e| format!("response error: {}", e))
        })
    });

    match result {
        Ok(text) => FnOutput::Text(text),
        Err(e) => FnOutput::error("skus", e),
    }
}
