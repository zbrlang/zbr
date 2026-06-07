use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, UserId};

/// ZmemberFlags{userID;flagName}
/// Flags: BYPASSES_VERIFICATION, DID_REJOIN, COMPLETED_ONBOARDING, STARTED_ONBOARDING
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let uid_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("memberFlags", crate::error_messages::required(1, "userID")),
    };
    let flag_name = match args.get(1) {
        Some(s) if !s.is_empty() => s.to_uppercase(),
        _ => return FnOutput::error("memberFlags", crate::error_messages::required(2, "flagName")),
    };

    let uid: u64 = match uid_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("memberFlags", crate::error_messages::expected_snowflake(1, "userID", &uid_str)),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("memberFlags", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("memberFlags", crate::error_messages::not_available("HTTP client")),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            http.get_member(GuildId::new(gid), UserId::new(uid)).await
        })
    });

    match result {
        Ok(member) => {
            let flags = member.flags;
            let has_flag = match flag_name.as_str() {
                "BYPASSES_VERIFICATION" => flags.contains(serenity::model::guild::GuildMemberFlags::BYPASSES_VERIFICATION),
                "DID_REJOIN" => flags.contains(serenity::model::guild::GuildMemberFlags::DID_REJOIN),
                "COMPLETED_ONBOARDING" => flags.contains(serenity::model::guild::GuildMemberFlags::COMPLETED_ONBOARDING),
                "STARTED_ONBOARDING" => flags.contains(serenity::model::guild::GuildMemberFlags::STARTED_ONBOARDING),
                _ => return FnOutput::error("memberFlags", crate::error_messages::expected_choice(2, "flagName", "BYPASSES_VERIFICATION, DID_REJOIN, COMPLETED_ONBOARDING, STARTED_ONBOARDING", &flag_name)),
            };

            FnOutput::Text(has_flag.to_string())
        }
        Err(_) => FnOutput::error("memberFlags", crate::error_messages::not_found("member", &uid_str)),
    }
}
