use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

/// ZisUserDMEnabled{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("isUserDMEnabled", "invalid userID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("isUserDMEnabled", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let user = UserId::new(uid).to_user(&http).await
                .map_err(|e| format!("failed to fetch user: {}", e))?;
            user.create_dm_channel(&http).await
                .map_err(|e| format!("failed to create dm: {}", e))
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(_) => FnOutput::Text("false".to_string()),
    }
}
