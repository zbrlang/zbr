use crate::context::{DiscordContext, FnOutput};

/// ZonboardingDefaultChannels
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onboardingDefaultChannels", crate::error_messages::not_available("HTTP client")),
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
                let ids = json["default_channel_ids"].as_array();
                if let Some(arr) = ids {
                    let id_strings: Vec<String> = arr.iter().filter_map(|id| {
                        if let Some(s) = id.as_str() {
                            Some(s.to_string())
                        } else if let Some(n) = id.as_u64() {
                            Some(n.to_string())
                        } else {
                            None
                        }
                    }).collect();
                    Ok(id_strings.join(","))
                } else {
                    Ok("".to_string())
                }
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("onboardingDefaultChannels", e),
    }
}
