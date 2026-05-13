use crate::context::{DiscordContext, FnOutput};

/// ZdeleteInvite{code}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let code = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("deleteInvite", "invite code is required"),
    };
    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("deleteInvite", "no HTTP client available"),
    };
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.delete_invite(&code, None)
                .await
                .map_err(|e| format!("failed to delete invite: {}", e))
        })
    });
    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("deleteInvite", e),
    }
}
