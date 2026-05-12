use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::{UserId, GuildId};
use crate::functions::permissions::helpers::member_guild_permissions;
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

/// ZuserPerms{userID;amount;separator}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let mut user_id_str = args.get(0).cloned().unwrap_or_else(|| ctx.author_id.clone());
    if user_id_str.is_empty() {
        user_id_str = ctx.author_id.clone();
    }
    let amount_str = args.get(1).cloned().unwrap_or_default();
    let separator = args.get(2).cloned().unwrap_or_else(|| "\n".to_string());

    let uid: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userPerms", "invalid userID"),
    };

    let gid: u64 = match ctx.guild_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("userPerms", "not in a guild"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("userPerms", "no HTTP client available"),
    };

    let amount: usize = if amount_str.is_empty() || amount_str == "all" {
        PERMS.len()
    } else {
        match amount_str.parse() {
            Ok(v) => v,
            Err(_) => return FnOutput::error("userPerms", "invalid amount"),
        }
    };

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let member = GuildId::new(gid).member(&http, UserId::new(uid)).await
                .map_err(|e| format!("failed to fetch member: {}", e))?;
            member_guild_permissions(&http, gid, &member).await
        })
    });

    match result {
        Ok(user_perms) => {
            let mut list = Vec::new();
            for (name, flag) in PERMS {
                if user_perms.contains(*flag) {
                    list.push(name.to_string());
                }
            }
            list.truncate(amount);
            FnOutput::Text(list.join(&separator))
        }
        Err(_) => FnOutput::error("userPerms", "user not found"),
    }
}
