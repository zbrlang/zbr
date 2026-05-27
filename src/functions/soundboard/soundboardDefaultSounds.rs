use crate::context::{DiscordContext, FnOutput};

/// ZsoundboardDefaultSounds{}
pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("soundboardDefaultSounds", crate::error_messages::action_failed("get HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.list_default_soundboard_sounds()
                .await
                .map_err(|e| format!("{}", e))
        })
    });

    match result {
        Ok(sounds) => FnOutput::Text(serde_json::to_string(&sounds).unwrap_or_default()),
        Err(e) => FnOutput::error("soundboardDefaultSounds", e),
    }
}
