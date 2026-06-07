use crate::context::{DiscordContext, FnOutput};

/// ZcreateServerTemplate{name;description?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let name = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("createServerTemplate", crate::error_messages::required(1, "name")),
    };
    let description = args.get(1).map(|s| s.clone()).unwrap_or_default();

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("createServerTemplate", crate::error_messages::not_available("HTTP client")),
    };
    let token = http.token().to_string();
    let gid = &ctx.guild_id;

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let res = client
                .post(format!("https://discord.com/api/v10/guilds/{}/templates", gid))
                .header("Authorization", &token)
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "name": name,
                    "description": description
                }))
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?;
            
            if res.status().is_success() {
                let json: serde_json::Value = res.json().await.map_err(|e| format!("JSON error: {}", e))?;
                Ok(json["code"].as_str().unwrap_or_default().to_string())
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("createServerTemplate", e),
    }
}
