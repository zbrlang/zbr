use crate::context::{DiscordContext, FnOutput};

/// ZisMentioned{userID?}
/// Returns "true" if the user is mentioned in the triggering message.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };

    if uid_str.is_empty() {
        return FnOutput::Text("false".to_string());
    }

    // Match <@userID> or <@!userID> patterns in the message
    let mentioned = ctx.message.contains(&format!("<@{}>", uid_str))
        || ctx.message.contains(&format!("<@!{}>", uid_str));

    FnOutput::Text(mentioned.to_string())
}
