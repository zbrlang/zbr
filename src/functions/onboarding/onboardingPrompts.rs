use crate::context::{DiscordContext, FnOutput};

/// ZonboardingPrompts
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onboardingPrompts", crate::error_messages::not_available("HTTP client")),
    };
    let token = http.token().to_string();
    let gid = &ctx.guild_id;

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let res = client
                .get(format!("https://discord.com/api/v10/guilds/{}/onboarding", gid))
                .header("Authorization", &token)
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?;
            
            if res.status().is_success() {
                let json: serde_json::Value = res.json().await.map_err(|e| format!("JSON error: {}", e))?;
                let prompts = json["prompts"].clone();
                serde_json::to_string_pretty(&prompts).map_err(|e| format!("Serialization error: {}", e))
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("onboardingPrompts", e),
    }
}
