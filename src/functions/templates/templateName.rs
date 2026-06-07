use crate::context::{DiscordContext, FnOutput};

/// ZtemplateName{code}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let code = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("templateName", crate::error_messages::required(1, "code")),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("templateName", crate::error_messages::not_available("HTTP client")),
    };
    let token = http.token().to_string();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let res = client
                .get(format!("https://discord.com/api/v10/guilds/templates/{}", code))
                .header("Authorization", &token)
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?;
            
            if res.status().is_success() {
                let json: serde_json::Value = res.json().await.map_err(|e| format!("JSON error: {}", e))?;
                Ok(json["name"].as_str().unwrap_or_default().to_string())
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("templateName", e),
    }
}
