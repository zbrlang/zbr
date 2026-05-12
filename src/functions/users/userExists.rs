use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

/// ZuserExists{userID}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    if user_id_str.is_empty() {
        return FnOutput::error("userExists", "userID is required");
    }

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::Text("false".to_string()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userExists", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            UserId::new(uid).to_user(&http).await
        })
    });

    match result {
        Ok(_) => FnOutput::Text("true".to_string()),
        Err(_) => FnOutput::Text("false".to_string()),
    }
}
