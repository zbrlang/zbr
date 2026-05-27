use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{GuildId, RoleId, UserId};
use serenity::model::permissions::Permissions;

const PERMS: &[(&str, Permissions)] = &[
    ("administrator", Permissions::ADMINISTRATOR),
    ("manageGuild", Permissions::MANAGE_GUILD),
    ("manageChannels", Permissions::MANAGE_CHANNELS),
    ("manageRoles", Permissions::MANAGE_ROLES),
    ("manageEmojis", Permissions::MANAGE_GUILD_EXPRESSIONS),
    ("manageMessages", Permissions::MANAGE_MESSAGES),
    ("manageWebhooks", Permissions::MANAGE_WEBHOOKS),
    ("manageNicknames", Permissions::MANAGE_NICKNAMES),
    ("kickMembers", Permissions::KICK_MEMBERS),
    ("banMembers", Permissions::BAN_MEMBERS),
    ("mentionEveryone", Permissions::MENTION_EVERYONE),
    ("sendMessages", Permissions::SEND_MESSAGES),
    ("viewChannel", Permissions::VIEW_CHANNEL),
    ("readMessageHistory", Permissions::READ_MESSAGE_HISTORY),
    ("embedLinks", Permissions::EMBED_LINKS),
    ("attachFiles", Permissions::ATTACH_FILES),
    ("addReactions", Permissions::ADD_REACTIONS),
    ("useExternalEmojis", Permissions::USE_EXTERNAL_EMOJIS),
    ("connect", Permissions::CONNECT),
    ("speak", Permissions::SPEAK),
    ("muteMembers", Permissions::MUTE_MEMBERS),
    ("deafenMembers", Permissions::DEAFEN_MEMBERS),
    ("moveMembers", Permissions::MOVE_MEMBERS),
    ("viewAuditLog", Permissions::VIEW_AUDIT_LOG),
    ("createInstantInvite", Permissions::CREATE_INSTANT_INVITE),
    ("prioritySpeaker", Permissions::PRIORITY_SPEAKER),
    ("stream", Permissions::STREAM),
    ("sendTTSMessages", Permissions::SEND_TTS_MESSAGES),
    ("useSlashCommands", Permissions::USE_APPLICATION_COMMANDS),
    ("requestToSpeak", Permissions::REQUEST_TO_SPEAK),
    ("manageEvents", Permissions::MANAGE_EVENTS),
    ("manageThreads", Permissions::MANAGE_THREADS),
    ("createPublicThreads", Permissions::CREATE_PUBLIC_THREADS),
    ("createPrivateThreads", Permissions::CREATE_PRIVATE_THREADS),
    ("timeout", Permissions::MODERATE_MEMBERS),
];

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let role_id_str = if args.get(0).map(|s| s.is_empty()).unwrap_or(true) {
        // fetch author's top role
        let http = ctx.http.as_ref().unwrap().clone();
        let guild_id = ctx.guild_id.parse::<u64>().map(GuildId::new).unwrap();
        let user_id = ctx.author_id.parse::<u64>().map(UserId::new).unwrap();
        let member = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async move { http.get_member(guild_id, user_id).await })
        });
        match member {
            Ok(m) => m.roles.first().map(|r| r.to_string()).unwrap_or_default(),
            Err(_) => return FnOutput::error("rolePerms", crate::error_messages::action_failed("get author's top role")),
        }
    } else {
        args[0].clone()
    };

    let separator = args.get(1).cloned().unwrap_or_else(|| "\n".to_string());

    let rid: u64 = match role_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("rolePerms", "invalid role ID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("rolePerms", crate::error_messages::not_in_guild()),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("rolePerms", "no HTTP client available"),
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let roles = GuildId::new(gid)
                .roles(&http)
                .await
                .map_err(|e| e.to_string())?;
            if let Some(role) = roles.get(&RoleId::new(rid)) {
                Ok(role.permissions)
            } else {
                Err("role not found".to_string())
            }
        })
    });

    match result {
        Ok(permissions) => {
            let mut list = Vec::new();
            for (name, flag) in PERMS {
                if permissions.contains(*flag) {
                    list.push(name.to_string());
                }
            }
            FnOutput::Text(list.join(&separator))
        }
        Err(e) => FnOutput::error("rolePerms", e),
    }
}
