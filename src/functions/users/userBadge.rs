use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::UserId;

/// ZuserBadge{userID;separator}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }
    let separator = args.get(1).cloned().unwrap_or_else(|| "\n".to_string());

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userBadge", "invalid userID"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userBadge", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            UserId::new(uid).to_user(&http).await
        })
    });

    match result {
        Ok(user) => {
            let mut badges = Vec::new();
            if let Some(flags) = user.public_flags {
                if flags.contains(serenity::model::user::UserPublicFlags::DISCORD_EMPLOYEE) { badges.push("Discord Staff"); }
                if flags.contains(serenity::model::user::UserPublicFlags::PARTNERED_SERVER_OWNER) { badges.push("Partnered Server Owner"); }
                if flags.contains(serenity::model::user::UserPublicFlags::HYPESQUAD_EVENTS) { badges.push("HypeSquad Events"); }
                if flags.contains(serenity::model::user::UserPublicFlags::BUG_HUNTER_LEVEL_1) { badges.push("Bug Hunter Level 1"); }
                if flags.contains(serenity::model::user::UserPublicFlags::HOUSE_BRAVERY) { badges.push("HypeSquad Bravery"); }
                if flags.contains(serenity::model::user::UserPublicFlags::HOUSE_BRILLIANCE) { badges.push("HypeSquad Brilliance"); }
                if flags.contains(serenity::model::user::UserPublicFlags::HOUSE_BALANCE) { badges.push("HypeSquad Balance"); }
                if flags.contains(serenity::model::user::UserPublicFlags::EARLY_SUPPORTER) { badges.push("Early Supporter"); }
                if flags.contains(serenity::model::user::UserPublicFlags::TEAM_USER) { badges.push("Team User"); }
                if flags.contains(serenity::model::user::UserPublicFlags::BUG_HUNTER_LEVEL_2) { badges.push("Bug Hunter Level 2"); }
                if flags.contains(serenity::model::user::UserPublicFlags::VERIFIED_BOT) { badges.push("Verified Bot"); }
                if flags.contains(serenity::model::user::UserPublicFlags::EARLY_VERIFIED_BOT_DEVELOPER) { badges.push("Verified Bot Developer"); }
                if flags.contains(serenity::model::user::UserPublicFlags::DISCORD_CERTIFIED_MODERATOR) { badges.push("Discord Certified Moderator"); }
                if flags.contains(serenity::model::user::UserPublicFlags::BOT_HTTP_INTERACTIONS) { badges.push("Bot HTTP Interactions"); }
                if flags.contains(serenity::model::user::UserPublicFlags::ACTIVE_DEVELOPER) { badges.push("Active Developer"); }
            }
            FnOutput::Text(badges.join(&separator))
        }
        Err(e) => FnOutput::error("userBadge", format!("failed to fetch user: {}", e)),
    }
}
