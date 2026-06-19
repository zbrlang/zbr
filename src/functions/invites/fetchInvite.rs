use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h,
        None => return FnOutput::error("fetchInvite", "no HTTP client available"),
    };

    let code = match args.get(0).filter(|s| !s.is_empty()) {
        Some(c) => c,
        None => return FnOutput::error("fetchInvite", crate::error_messages::required(1, "code")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            match http.get_invite(code, false, false, None).await {
                Ok(invite) => {
                    serde_json::to_string(&invite).map_err(|e| e.to_string())
                }
                Err(e) => Err(format!("fetchInvite error: {}", e)),
            }
        })
    });

    match result {
        Err(e) => FnOutput::Error(e),
        Ok(json) => FnOutput::Text(json),
    }
}
