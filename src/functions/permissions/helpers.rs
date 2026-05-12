use serenity::model::permissions::Permissions;
use serenity::model::guild::Member;
use serenity::http::Http;
use serenity::model::id::GuildId;
use std::sync::Arc;

/// Parse a permission name string into a serenity Permissions bitflag.
/// Case-insensitive. Returns None if unrecognised.
pub fn parse_permission(s: &str) -> Option<Permissions> {
    match s.trim().to_lowercase().replace([' ', '_', '-'], "").as_str() {
        "administrator"                          => Some(Permissions::ADMINISTRATOR),
        "manageguild" | "manageserver"           => Some(Permissions::MANAGE_GUILD),
        "managechannels"                         => Some(Permissions::MANAGE_CHANNELS),
        "manageroles"                            => Some(Permissions::MANAGE_ROLES),
        "manageemojis" | "manageguildexpressions"=> Some(Permissions::MANAGE_GUILD_EXPRESSIONS),
        "managemessages"                         => Some(Permissions::MANAGE_MESSAGES),
        "managewebhooks"                         => Some(Permissions::MANAGE_WEBHOOKS),
        "managenicknames"                        => Some(Permissions::MANAGE_NICKNAMES),
        "kickmembers"                            => Some(Permissions::KICK_MEMBERS),
        "banmembers"                             => Some(Permissions::BAN_MEMBERS),
        "mentioneveryone"                        => Some(Permissions::MENTION_EVERYONE),
        "sendmessages"                           => Some(Permissions::SEND_MESSAGES),
        "readmessages" | "viewchannel"           => Some(Permissions::VIEW_CHANNEL),
        "readmessagehistory"                     => Some(Permissions::READ_MESSAGE_HISTORY),
        "embedlinks"                             => Some(Permissions::EMBED_LINKS),
        "attachfiles"                            => Some(Permissions::ATTACH_FILES),
        "addreactions"                           => Some(Permissions::ADD_REACTIONS),
        "usexternalemojis"                       => Some(Permissions::USE_EXTERNAL_EMOJIS),
        "connect"                                => Some(Permissions::CONNECT),
        "speak"                                  => Some(Permissions::SPEAK),
        "mutemembers"                            => Some(Permissions::MUTE_MEMBERS),
        "deafenmembers"                          => Some(Permissions::DEAFEN_MEMBERS),
        "movemembers"                            => Some(Permissions::MOVE_MEMBERS),
        "viewauditlog"                           => Some(Permissions::VIEW_AUDIT_LOG),
        "createinstantinvite"                    => Some(Permissions::CREATE_INSTANT_INVITE),
        "priorityspeaker"                        => Some(Permissions::PRIORITY_SPEAKER),
        "stream"                                 => Some(Permissions::STREAM),
        "sendttsmessages"                        => Some(Permissions::SEND_TTS_MESSAGES),
        "useapplicationcommands" | "useslashcommands" => Some(Permissions::USE_APPLICATION_COMMANDS),
        "requesttospeak"                         => Some(Permissions::REQUEST_TO_SPEAK),
        "manageevents"                           => Some(Permissions::MANAGE_EVENTS),
        "managethreads"                          => Some(Permissions::MANAGE_THREADS),
        "createpublicthreads"                    => Some(Permissions::CREATE_PUBLIC_THREADS),
        "createprivatethreads"                   => Some(Permissions::CREATE_PRIVATE_THREADS),
        "moderatemembers" | "timeout"            => Some(Permissions::MODERATE_MEMBERS),
        _ => None,
    }
}

/// Parse multiple permission name args into a combined bitflag.
pub fn parse_permissions(args: &[String]) -> Result<Permissions, String> {
    let mut combined = Permissions::empty();
    for arg in args {
        match parse_permission(arg) {
            Some(p) => combined |= p,
            None => return Err(format!("unknown permission: '{}'", arg)),
        }
    }
    Ok(combined)
}

/// Compute a member's effective guild permissions by OR-ing all their role permissions.
/// If any role has ADMINISTRATOR, returns Permissions::all().
pub async fn member_guild_permissions(
    http: &Arc<Http>,
    guild_id: u64,
    member: &Member,
) -> Result<Permissions, String> {
    let roles = GuildId::new(guild_id).roles(http).await
        .map_err(|e| format!("failed to fetch roles: {}", e))?;

    // Check if guild owner
    let guild = GuildId::new(guild_id).to_partial_guild(http).await
        .map_err(|e| format!("failed to fetch guild: {}", e))?;

    if guild.owner_id == member.user.id {
        return Ok(Permissions::all());
    }

    let mut perms = Permissions::empty();

    // @everyone role has the same ID as the guild
    if let Some(everyone) = roles.get(&serenity::model::id::RoleId::new(guild_id)) {
        perms |= everyone.permissions;
    }

    for role_id in &member.roles {
        if let Some(role) = roles.get(role_id) {
            perms |= role.permissions;
        }
    }

    if perms.contains(Permissions::ADMINISTRATOR) {
        return Ok(Permissions::all());
    }

    Ok(perms)
}
