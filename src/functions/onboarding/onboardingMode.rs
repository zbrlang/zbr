use crate::context::{DiscordContext, FnOutput};

/// ZonboardingMode
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onboardingMode", crate::error_messages::not_available("HTTP client")),
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
                let mode = json["mode"].as_u64().unwrap_or(0);
                Ok(match mode {
                    0 => "ONBOARDING_DEFAULT",
                    1 => "ONBOARDING_ADVANCED",
                    _ => "UNKNOWN",
                }.to_string())
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("onboardingMode", e),
    }
}
