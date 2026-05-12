use crate::context::{DiscordContext, FnOutput};
use serenity::model::gateway::ActivityType;
use serenity::model::id::{GuildId, UserId};
use serenity::model::user::OnlineStatus;

/// ZuserStatus{userID;customStatus}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => ctx.author_id.clone(),
    };
    let custom_status = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => "false".to_string(),
    };

    let user_id = match user_id_str.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return FnOutput::error("userStatus", "invalid user ID"),
    };

    let guild_id = match ctx.guild_id.parse::<u64>() {
        Ok(id) => GuildId::new(id),
        Err(_) => return FnOutput::Text("offline".to_string()), // not in guild, offline
    };

    let presence = ctx
        .cache
        .guild(guild_id)
        .and_then(|g| g.presences.get(&user_id).cloned());

    match presence {
        Some(p) => {
            if custom_status == "true" {
                for activity in &p.activities {
                    if activity.kind == ActivityType::Custom {
                        return FnOutput::Text(activity.state.clone().unwrap_or_default());
                    }
                }
                FnOutput::Text("".to_string())
            } else {
                let status = match p.status {
                    OnlineStatus::Online => "online",
                    OnlineStatus::Offline => "offline",
                    OnlineStatus::Idle => "idle",
                    OnlineStatus::DoNotDisturb => "dnd",
                    _ => "offline",
                };
                FnOutput::Text(status.to_string())
            }
        }
        None => FnOutput::Text("offline".to_string()),
    }
}
