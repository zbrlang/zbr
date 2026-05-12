use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

/// ZisBot{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isBot", "invalid userID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isBot", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            UserId::new(uid).to_user(&http).await
        })
    });

    match result {
        Ok(user) => {
            if user.bot {
                FnOutput::Text("true".to_string())
            } else {
                FnOutput::Text("false".to_string())
            }
        }
        Err(e) => FnOutput::error("isBot", format!("failed to fetch user: {}", e)),
    }
}
