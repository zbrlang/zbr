use crate::context::{DiscordContext, FnOutput};
use serenity::model::permissions::Permissions;

/// ZpermNames{bitfield}
/// Takes a permission bitfield integer. Returns a comma-separated list of human-readable permission names.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("permNames", crate::error_messages::required(1, "bitfield"));
    }

    let bits: u64 = match args[0].parse() {
        Ok(b) => b,
        Err(_) => return FnOutput::error("permNames", crate::error_messages::expected_integer(1, "bitfield", &args[0])),
    };

    let perms = Permissions::from_bits_truncate(bits);
    let mut names = Vec::new();

    if perms.contains(Permissions::ADMINISTRATOR) { names.push("Administrator"); }
    if perms.contains(Permissions::CREATE_INSTANT_INVITE) { names.push("Create Instant Invite"); }
    if perms.contains(Permissions::KICK_MEMBERS) { names.push("Kick Members"); }
    if perms.contains(Permissions::BAN_MEMBERS) { names.push("Ban Members"); }
    if perms.contains(Permissions::MANAGE_CHANNELS) { names.push("Manage Channels"); }
    if perms.contains(Permissions::MANAGE_GUILD) { names.push("Manage Server"); }
    if perms.contains(Permissions::ADD_REACTIONS) { names.push("Add Reactions"); }
    if perms.contains(Permissions::VIEW_AUDIT_LOG) { names.push("View Audit Log"); }
    if perms.contains(Permissions::PRIORITY_SPEAKER) { names.push("Priority Speaker"); }
    if perms.contains(Permissions::STREAM) { names.push("Stream"); }
    if perms.contains(Permissions::VIEW_CHANNEL) { names.push("View Channel"); }
    if perms.contains(Permissions::SEND_MESSAGES) { names.push("Send Messages"); }
    if perms.contains(Permissions::SEND_TTS_MESSAGES) { names.push("Send TTS Messages"); }
    if perms.contains(Permissions::MANAGE_MESSAGES) { names.push("Manage Messages"); }
    if perms.contains(Permissions::EMBED_LINKS) { names.push("Embed Links"); }
    if perms.contains(Permissions::ATTACH_FILES) { names.push("Attach Files"); }
    if perms.contains(Permissions::READ_MESSAGE_HISTORY) { names.push("Read Message History"); }
    if perms.contains(Permissions::MENTION_EVERYONE) { names.push("Mention Everyone"); }
    if perms.contains(Permissions::USE_EXTERNAL_EMOJIS) { names.push("Use External Emojis"); }
    if perms.contains(Permissions::VIEW_GUILD_INSIGHTS) { names.push("View Server Insights"); }
    if perms.contains(Permissions::CONNECT) { names.push("Connect"); }
    if perms.contains(Permissions::SPEAK) { names.push("Speak"); }
    if perms.contains(Permissions::MUTE_MEMBERS) { names.push("Mute Members"); }
    if perms.contains(Permissions::DEAFEN_MEMBERS) { names.push("Deafen Members"); }
    if perms.contains(Permissions::MOVE_MEMBERS) { names.push("Move Members"); }
    if perms.contains(Permissions::USE_VAD) { names.push("Use Voice Activity"); }
    if perms.contains(Permissions::CHANGE_NICKNAME) { names.push("Change Nickname"); }
    if perms.contains(Permissions::MANAGE_NICKNAMES) { names.push("Manage Nicknames"); }
    if perms.contains(Permissions::MANAGE_ROLES) { names.push("Manage Roles"); }
    if perms.contains(Permissions::MANAGE_WEBHOOKS) { names.push("Manage Webhooks"); }
    if perms.contains(Permissions::MANAGE_GUILD_EXPRESSIONS) { names.push("Manage Emojis and Stickers"); }
    if perms.contains(Permissions::USE_APPLICATION_COMMANDS) { names.push("Use Application Commands"); }
    if perms.contains(Permissions::REQUEST_TO_SPEAK) { names.push("Request to Speak"); }
    if perms.contains(Permissions::MANAGE_EVENTS) { names.push("Manage Events"); }
    if perms.contains(Permissions::MANAGE_THREADS) { names.push("Manage Threads"); }
    if perms.contains(Permissions::CREATE_PUBLIC_THREADS) { names.push("Create Public Threads"); }
    if perms.contains(Permissions::CREATE_PRIVATE_THREADS) { names.push("Create Private Threads"); }
    if perms.contains(Permissions::USE_EXTERNAL_STICKERS) { names.push("Use External Stickers"); }
    if perms.contains(Permissions::SEND_MESSAGES_IN_THREADS) { names.push("Send Messages in Threads"); }
    if perms.contains(Permissions::USE_EMBEDDED_ACTIVITIES) { names.push("Use Embedded Activities"); }
    if perms.contains(Permissions::MODERATE_MEMBERS) { names.push("Timeout Members"); }
    if perms.contains(Permissions::VIEW_CREATOR_MONETIZATION_ANALYTICS) { names.push("View Creator Monetization Analytics"); }
    if perms.contains(Permissions::USE_SOUNDBOARD) { names.push("Use Soundboard"); }
    if perms.contains(Permissions::USE_EXTERNAL_SOUNDS) { names.push("Use External Sounds"); }
    if perms.contains(Permissions::SEND_VOICE_MESSAGES) { names.push("Send Voice Messages"); }

    FnOutput::Text(names.join(", "))
}
