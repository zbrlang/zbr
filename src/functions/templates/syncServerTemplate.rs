use crate::context::{DiscordContext, FnOutput};

/// ZsyncServerTemplate{code}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let code = match args.get(0) {
        Some(s) if !s.is_empty() => s,
        _ => return FnOutput::error("syncServerTemplate", crate::error_messages::required(1, "code")),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("syncServerTemplate", crate::error_messages::not_available("HTTP client")),
    };
    let token = http.token().to_string();
    let gid = &ctx.guild_id;

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let client = reqwest::Client::new();
            let res = client
                .put(format!("https://discord.com/api/v10/guilds/{}/templates/{}/sync", gid, code))
                .header("Authorization", &token)
                .send()
                .await
                .map_err(|e| format!("HTTP error: {}", e))?;
            
            if res.status().is_success() {
                Ok("true".to_string())
            } else {
                Err(format!("HTTP error: {}", res.status()))
            }
        })
    });

    match result {
        Ok(val) => FnOutput::Text(val),
        Err(e) => FnOutput::error("syncServerTemplate", e),
    }
}
