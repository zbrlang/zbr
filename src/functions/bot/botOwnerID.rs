use crate::context::{DiscordContext, FnOutput};
use std::sync::OnceLock;

static OWNER_ID: OnceLock<String> = OnceLock::new();

pub fn run(_args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if let Some(id) = OWNER_ID.get() {
        return FnOutput::Text(id.clone());
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("botOwnerID", crate::error_messages::requires_set_first("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_current_application_info()
                .await
                .map(|info| {
                    if let Some(team) = info.team {
                        team.owner_user_id.to_string()
                    } else if let Some(owner) = info.owner {
                        owner.id.to_string()
                    } else {
                        String::new()
                    }
                })
                .map_err(|e| e.to_string())
        })
    });

    match result {
        Ok(id) => {
            if !id.is_empty() {
                let _ = OWNER_ID.set(id.clone());
            }
            FnOutput::Text(id)
        }
        Err(e) => FnOutput::error("botOwnerID", crate::error_messages::action_failed_reason("fetch application info", &e)),
    }
}
