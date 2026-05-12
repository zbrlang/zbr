use crate::context::{DiscordContext, FnOutput};
use std::sync::OnceLock;

static OWNER_ID: OnceLock<String> = OnceLock::new();

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if let Some(id) = OWNER_ID.get() {
        return FnOutput::Text(id.clone());
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("botOwnerID", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_current_application_info()
                .await
                .map(|info| info.owner.map(|u| u.id.to_string()).unwrap_or_default())
                .map_err(|e| format!("failed to fetch application info: {}", e))
        })
    });

    match result {
        Ok(id) => {
            if !id.is_empty() {
                let _ = OWNER_ID.set(id.clone());
            }
            FnOutput::Text(id)
        }
        Err(e) => FnOutput::error("botOwnerID", e),
    }
}
