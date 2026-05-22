#![allow(non_snake_case)]

pub mod actions;
pub mod blacklists;
pub mod bot;
pub mod channels;
pub mod components;
pub mod context;
pub mod control;
pub mod cooldown;
pub mod embeds;
pub mod emojis;
pub mod errors;
pub mod events;
pub mod forums;
pub mod audit;
pub mod http;
pub mod id;
pub mod invites;
pub mod json;
pub mod loops;
pub mod math;
pub mod message;
pub mod moderation;
pub mod permissions;
pub mod random;
pub mod reactions;
pub mod roles;
pub mod servers;
pub mod stage;
pub mod stickers;
pub mod string;
pub mod textsplit;
pub mod threads;
pub mod time;
pub mod users;
pub mod variables;
pub mod voice;

use crate::context::FnMeta;
use crate::context::FnOutput;
use std::collections::HashMap;

pub fn register(registry: &mut HashMap<String, FnMeta>) {
    // context
    registry.insert(
        "username".to_string(),
        FnMeta {
            func: users::username::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "message".to_string(),
        FnMeta {
            func: context::message::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "option".to_string(),
        FnMeta {
            func: context::option::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // id
    registry.insert(
        "userID".to_string(),
        FnMeta {
            func: id::userID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelID".to_string(),
        FnMeta {
            func: id::channelID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "guildID".to_string(),
        FnMeta {
            func: id::guildID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "roleID".to_string(),
        FnMeta {
            func: id::roleID::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // time
    registry.insert(
        "time".to_string(),
        FnMeta {
            func: time::time::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "date".to_string(),
        FnMeta {
            func: time::date::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "day".to_string(),
        FnMeta {
            func: time::day::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "month".to_string(),
        FnMeta {
            func: time::month::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "year".to_string(),
        FnMeta {
            func: time::year::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "hour".to_string(),
        FnMeta {
            func: time::hour::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "minute".to_string(),
        FnMeta {
            func: time::minute::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "second".to_string(),
        FnMeta {
            func: time::second::run,
            min_args: 0,
            max_args: 0,
        },
    );

    // permissions
    registry.insert(
        "checkUserPerms".to_string(),
        FnMeta {
            func: permissions::checkUserPerms::run,
            min_args: 2,
            max_args: 50,
        },
    );
    registry.insert(
        "ignoreChannels".to_string(),
        FnMeta {
            func: permissions::ignoreChannels::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyAdmin".to_string(),
        FnMeta {
            func: permissions::onlyAdmin::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "onlyBotChannelPerms".to_string(),
        FnMeta {
            func: permissions::onlyBotChannelPerms::run,
            min_args: 2,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyBotPerms".to_string(),
        FnMeta {
            func: permissions::onlyBotPerms::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForCategories".to_string(),
        FnMeta {
            func: permissions::onlyForCategories::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForChannels".to_string(),
        FnMeta {
            func: permissions::onlyForChannels::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForIDs".to_string(),
        FnMeta {
            func: permissions::onlyForIDs::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForRoles".to_string(),
        FnMeta {
            func: permissions::onlyForRoles::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForRoleIDs".to_string(),
        FnMeta {
            func: permissions::onlyForRoleIDs::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForServers".to_string(),
        FnMeta {
            func: permissions::onlyForServers::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyForUsers".to_string(),
        FnMeta {
            func: permissions::onlyForUsers::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyNSFW".to_string(),
        FnMeta {
            func: permissions::onlyNSFW::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "onlyPerms".to_string(),
        FnMeta {
            func: permissions::onlyPerms::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "onlyIfMessageContains".to_string(),
        FnMeta {
            func: permissions::onlyIfMessageContains::run,
            min_args: 3,
            max_args: 100,
        },
    );

    // textsplit
    registry.insert(
        "textSplit".to_string(),
        FnMeta {
            func: textsplit::textSplit::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "splitText".to_string(),
        FnMeta {
            func: textsplit::splitText::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "getTextSplitIndex".to_string(),
        FnMeta {
            func: textsplit::getTextSplitIndex::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "getTextSplitLength".to_string(),
        FnMeta {
            func: textsplit::getTextSplitLength::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "joinSplitText".to_string(),
        FnMeta {
            func: textsplit::joinSplitText::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "removeSplitTextElement".to_string(),
        FnMeta {
            func: textsplit::removeSplitTextElement::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "editSplitText".to_string(),
        FnMeta {
            func: textsplit::editSplitText::run,
            min_args: 2,
            max_args: 2,
        },
    );

    // string
    registry.insert(
        "lowercase".to_string(),
        FnMeta {
            func: string::lowercase::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "uppercase".to_string(),
        FnMeta {
            func: string::uppercase::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "charCount".to_string(),
        FnMeta {
            func: string::charCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "linesCount".to_string(),
        FnMeta {
            func: string::linesCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "numberSeparator".to_string(),
        FnMeta {
            func: string::numberSeparator::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "cropText".to_string(),
        FnMeta {
            func: string::cropText::run,
            min_args: 3,
            max_args: 3,
        },
    );
    registry.insert(
        "replaceText".to_string(),
        FnMeta {
            func: string::replaceText::run,
            min_args: 3,
            max_args: 4,
        },
    );
    registry.insert(
        "titlecase".to_string(),
        FnMeta {
            func: string::titlecase::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "trimContent".to_string(),
        FnMeta {
            func: string::trimContent::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "trimSpace".to_string(),
        FnMeta {
            func: string::trimSpace::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "url".to_string(),
        FnMeta {
            func: string::url::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "byteCount".to_string(),
        FnMeta {
            func: string::byteCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "argCount".to_string(),
        FnMeta {
            func: string::argCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "checkContains".to_string(),
        FnMeta {
            func: string::checkContains::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "removeLinks".to_string(),
        FnMeta {
            func: string::removeLinks::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // math
    registry.insert(
        "sum".to_string(),
        FnMeta {
            func: math::add::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "sub".to_string(),
        FnMeta {
            func: math::sub::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "div".to_string(),
        FnMeta {
            func: math::div::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "multi".to_string(),
        FnMeta {
            func: math::multi::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "calculate".to_string(),
        FnMeta {
            func: math::calculate::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "ceil".to_string(),
        FnMeta {
            func: math::ceil::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "floor".to_string(),
        FnMeta {
            func: math::floor::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "max".to_string(),
        FnMeta {
            func: math::max::run,
            min_args: 2,
            max_args: 100,
        },
    );

    // random
    registry.insert(
        "random".to_string(),
        FnMeta {
            func: random::random::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "randomCategoryID".to_string(),
        FnMeta {
            func: random::randomCategoryID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomChannelID".to_string(),
        FnMeta {
            func: random::randomChannelID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomGuildID".to_string(),
        FnMeta {
            func: random::randomGuildID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomMention".to_string(),
        FnMeta {
            func: random::randomMention::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomRoleID".to_string(),
        FnMeta {
            func: random::randomRoleID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomString".to_string(),
        FnMeta {
            func: random::randomString::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "randomText".to_string(),
        FnMeta {
            func: random::randomText::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "randomUser".to_string(),
        FnMeta {
            func: random::randomUser::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "randomUserID".to_string(),
        FnMeta {
            func: random::randomUserID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "min".to_string(),
        FnMeta {
            func: math::min::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "modulo".to_string(),
        FnMeta {
            func: math::modulo::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "sqrt".to_string(),
        FnMeta {
            func: math::sqrt::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "sort".to_string(),
        FnMeta {
            func: math::sort::run,
            min_args: 5,
            max_args: 103,
        },
    );

    // control
    registry.insert(
        "if".to_string(),
        FnMeta {
            func: control::zif::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "not".to_string(),
        FnMeta {
            func: control::not::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "checkCondition".to_string(),
        FnMeta {
            func: control::checkCondition::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "onlyIf".to_string(),
        FnMeta {
            func: control::onlyIf::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "argsCheck".to_string(),
        FnMeta {
            func: control::argsCheck::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "isBoolean".to_string(),
        FnMeta {
            func: control::isBoolean::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "isInteger".to_string(),
        FnMeta {
            func: control::isInteger::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "isNumber".to_string(),
        FnMeta {
            func: control::isNumber::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "isSlash".to_string(),
        FnMeta {
            func: control::isSlash::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "isValidHex".to_string(),
        FnMeta {
            func: control::isValidHex::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stop".to_string(),
        FnMeta {
            func: errors::stop::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "error".to_string(),
        FnMeta {
            func: errors::error::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "suppressErrors".to_string(),
        FnMeta {
            func: errors::suppressErrors::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "tryRun".to_string(),
        FnMeta {
            func: errors::tryRun::run,
            min_args: 1,
            max_args: 2,
        },
    );

    // cooldown
    registry.insert(
        "cooldown".to_string(),
        FnMeta {
            func: cooldown::cooldown::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "serverCooldown".to_string(),
        FnMeta {
            func: cooldown::serverCooldown::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "globalCooldown".to_string(),
        FnMeta {
            func: cooldown::globalCooldown::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "getCooldown".to_string(),
        FnMeta {
            func: cooldown::getCooldown::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "changeCooldownTime".to_string(),
        FnMeta {
            func: cooldown::changeCooldownTime::run,
            min_args: 4,
            max_args: 4,
        },
    );

    // emojis
    registry.insert(
        "addEmoji".to_string(),
        FnMeta {
            func: emojis::addEmoji::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "customEmoji".to_string(),
        FnMeta {
            func: emojis::customEmoji::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "emoteCount".to_string(),
        FnMeta {
            func: emojis::emoteCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "emojiExists".to_string(),
        FnMeta {
            func: emojis::emojiExists::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "emojiName".to_string(),
        FnMeta {
            func: emojis::emojiName::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "isEmojiAnimated".to_string(),
        FnMeta {
            func: emojis::isEmojiAnimated::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "removeEmoji".to_string(),
        FnMeta {
            func: emojis::removeEmoji::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // reactions
    registry.insert(
        "addReactions".to_string(),
        FnMeta {
            func: reactions::addReactions::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "addCmdReactions".to_string(),
        FnMeta {
            func: reactions::addCmdReactions::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "addMessageReactions".to_string(),
        FnMeta {
            func: reactions::addMessageReactions::run,
            min_args: 3,
            max_args: 102,
        },
    );
    registry.insert(
        "clearReactions".to_string(),
        FnMeta {
            func: reactions::clearReactions::run,
            min_args: 3,
            max_args: 3,
        },
    );
    registry.insert(
        "getReactions".to_string(),
        FnMeta {
            func: reactions::getReactions::run,
            min_args: 4,
            max_args: 4,
        },
    );
    registry.insert(
        "userReacted".to_string(),
        FnMeta {
            func: reactions::userReacted::run,
            min_args: 4,
            max_args: 4,
        },
    );

    // blacklists
    registry.insert(
        "blackListIDs".to_string(),
        FnMeta {
            func: blacklists::blackListIDs::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "blackListUsers".to_string(),
        FnMeta {
            func: blacklists::blackListUsers::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "blackListRoles".to_string(),
        FnMeta {
            func: blacklists::blackListRoles::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "blackListRolesIDs".to_string(),
        FnMeta {
            func: blacklists::blackListRolesIDs::run,
            min_args: 1,
            max_args: 50,
        },
    );
    registry.insert(
        "blackListServers".to_string(),
        FnMeta {
            func: blacklists::blackListServers::run,
            min_args: 1,
            max_args: 50,
        },
    );

    // threads
    registry.insert(
        "startThread".to_string(),
        FnMeta {
            func: threads::startThread::run,
            min_args: 2,
            max_args: 5,
        },
    );
    registry.insert(
        "editThread".to_string(),
        FnMeta {
            func: threads::editThread::run,
            min_args: 1,
            max_args: 6,
        },
    );
    registry.insert(
        "threadAddMember".to_string(),
        FnMeta {
            func: threads::threadAddMember::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "threadRemoveMember".to_string(),
        FnMeta {
            func: threads::threadRemoveMember::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "threadMessageCount".to_string(),
        FnMeta {
            func: threads::threadMessageCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "threadUserCount".to_string(),
        FnMeta {
            func: threads::threadUserCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "threadArchived".to_string(),
        FnMeta {
            func: threads::threadArchived::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "threadLocked".to_string(),
        FnMeta {
            func: threads::threadLocked::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "threadParentID".to_string(),
        FnMeta {
            func: threads::threadParentID::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Reactions
    registry.insert(
        "addReactions".to_string(),
        FnMeta {
            func: reactions::addReactions::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "addCmdReactions".to_string(),
        FnMeta {
            func: reactions::addCmdReactions::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "addMessageReactions".to_string(),
        FnMeta {
            func: reactions::addMessageReactions::run,
            min_args: 3,
            max_args: 100,
        },
    );
    registry.insert(
        "getReactions".to_string(),
        FnMeta {
            func: reactions::getReactions::run,
            min_args: 4,
            max_args: 4,
        },
    );
    registry.insert(
        "clearReactions".to_string(),
        FnMeta {
            func: reactions::clearReactions::run,
            min_args: 3,
            max_args: 100,
        },
    );
    registry.insert(
        "userReacted".to_string(),
        FnMeta {
            func: reactions::userReacted::run,
            min_args: 4,
            max_args: 4,
        },
    );

    // Time extras
    registry.insert(
        "getTimestamp".to_string(),
        FnMeta {
            func: time::getTimestamp::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // actions
    registry.insert(
        "reply".to_string(),
        FnMeta {
            func: actions::reply::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "sendMessage".to_string(),
        FnMeta {
            func: actions::sendMessage::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "replyIn".to_string(),
        FnMeta {
            func: actions::replyIn::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "allowUserMentions".to_string(),
        FnMeta {
            func: context::allowUserMentions::run,
            min_args: 0,
            max_args: 100,
        },
    );
    registry.insert(
        "allowRoleMentions".to_string(),
        FnMeta {
            func: context::allowRoleMentions::run,
            min_args: 0,
            max_args: 100,
        },
    );
    // eval is handled as a special case in Runtime::evaluate — registered here
    // only so arg-count checking fires correctly (0–1 args).
    registry.insert(
        "eval".to_string(),
        FnMeta {
            func: |_, _| FnOutput::Empty,
            min_args: 0,
            max_args: 1,
        },
    );

    // variables
    registry.insert(
        "getUserVar".to_string(),
        FnMeta {
            func: variables::getUserVar::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "setUserVar".to_string(),
        FnMeta {
            func: variables::setUserVar::run,
            min_args: 2,
            max_args: 4,
        },
    );
    registry.insert(
        "getServerVar".to_string(),
        FnMeta {
            func: variables::getServerVar::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "setServerVar".to_string(),
        FnMeta {
            func: variables::setServerVar::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "getChannelVar".to_string(),
        FnMeta {
            func: variables::getChannelVar::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "setChannelVar".to_string(),
        FnMeta {
            func: variables::setChannelVar::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "getVar".to_string(),
        FnMeta {
            func: variables::getVar::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "setVar".to_string(),
        FnMeta {
            func: variables::setVar::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "var".to_string(),
        FnMeta {
            func: variables::var::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "listVar".to_string(),
        FnMeta {
            func: variables::listVar::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "varExists".to_string(),
        FnMeta {
            func: variables::varExists::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "resetUserVar".to_string(),
        FnMeta {
            func: variables::resetUserVar::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "resetServerVar".to_string(),
        FnMeta {
            func: variables::resetServerVar::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "resetChannelVar".to_string(),
        FnMeta {
            func: variables::resetChannelVar::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "varExistError".to_string(),
        FnMeta {
            func: variables::varExistError::run,
            min_args: 2,
            max_args: 2,
        },
    );

    // embeds — all setters take an optional trailing index arg (1-based, default 1)
    registry.insert(
        "title".to_string(),
        FnMeta {
            func: embeds::title::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "titleURL".to_string(),
        FnMeta {
            func: embeds::titleURL::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "description".to_string(),
        FnMeta {
            func: embeds::description::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "color".to_string(),
        FnMeta {
            func: embeds::color::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "footer".to_string(),
        FnMeta {
            func: embeds::footer::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "footerIcon".to_string(),
        FnMeta {
            func: embeds::footerIcon::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "image".to_string(),
        FnMeta {
            func: embeds::image::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "thumbnail".to_string(),
        FnMeta {
            func: embeds::thumbnail::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "author".to_string(),
        FnMeta {
            func: embeds::author::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "authorIcon".to_string(),
        FnMeta {
            func: embeds::authorIcon::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "authorURL".to_string(),
        FnMeta {
            func: embeds::authorURL::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "timestamp".to_string(),
        FnMeta {
            func: embeds::timestamp::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "addField".to_string(),
        FnMeta {
            func: embeds::addField::run,
            min_args: 2,
            max_args: 4,
        },
    );
    registry.insert(
        "sendEmbed".to_string(),
        FnMeta {
            func: embeds::sendEmbed::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "webhookCreate".to_string(),
        FnMeta {
            func: embeds::webhookCreate::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "webhookDelete".to_string(),
        FnMeta {
            func: embeds::webhookDelete::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "sendWebhook".to_string(),
        FnMeta {
            func: embeds::sendWebhook::run,
            min_args: 1,
            max_args: 3,
        },
    );

    // users
    registry.insert(
        "userAvatar".to_string(),
        FnMeta {
            func: users::userAvatar::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userServerAvatar".to_string(),
        FnMeta {
            func: users::userServerAvatar::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userBadge".to_string(),
        FnMeta {
            func: users::userBadge::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "userBanner".to_string(),
        FnMeta {
            func: users::userBanner::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userBannerColor".to_string(),
        FnMeta {
            func: users::userBannerColor::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userExists".to_string(),
        FnMeta {
            func: users::userExists::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "creationDate".to_string(),
        FnMeta {
            func: users::creationDate::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "userPerms".to_string(),
        FnMeta {
            func: users::userPerms::run,
            min_args: 0,
            max_args: 3,
        },
    );
    registry.insert(
        "changeNickname".to_string(),
        FnMeta {
            func: users::changeNickname::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "displayName".to_string(),
        FnMeta {
            func: users::displayName::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "userStatus".to_string(),
        FnMeta {
            func: users::userStatus::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "isAdmin".to_string(),
        FnMeta {
            func: users::isAdmin::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "isBooster".to_string(),
        FnMeta {
            func: users::isBooster::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "isBot".to_string(),
        FnMeta {
            func: users::isBot::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "isUserDMEnabled".to_string(),
        FnMeta {
            func: users::isUserDMEnabled::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userJoined".to_string(),
        FnMeta {
            func: users::userJoined::run,
            min_args: 0,
            max_args: 2,
        },
    );

    // Channels
    registry.insert(
        "channelName".to_string(),
        FnMeta {
            func: channels::channelName::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelCount".to_string(),
        FnMeta {
            func: channels::channelCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "channelExists".to_string(),
        FnMeta {
            func: channels::channelExists::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "channelPosition".to_string(),
        FnMeta {
            func: channels::channelPosition::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelTopic".to_string(),
        FnMeta {
            func: channels::channelTopic::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelType".to_string(),
        FnMeta {
            func: channels::channelType::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelNames".to_string(),
        FnMeta {
            func: channels::channelNames::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "serverChannels".to_string(),
        FnMeta {
            func: channels::serverChannels::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "categoryID".to_string(),
        FnMeta {
            func: channels::categoryID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "categoryCount".to_string(),
        FnMeta {
            func: channels::categoryCount::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "categoryChannels".to_string(),
        FnMeta {
            func: channels::categoryChannels::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "afkChannelID".to_string(),
        FnMeta {
            func: channels::afkChannelID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "rulesChannelID".to_string(),
        FnMeta {
            func: channels::rulesChannelID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "systemChannelID".to_string(),
        FnMeta {
            func: channels::systemChannelID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "dmChannelID".to_string(),
        FnMeta {
            func: channels::dmChannelID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "parentID".to_string(),
        FnMeta {
            func: channels::parentID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "isNSFW".to_string(),
        FnMeta {
            func: channels::isNSFW::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "lastMessageID".to_string(),
        FnMeta {
            func: channels::lastMessageID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "lastPinTimestamp".to_string(),
        FnMeta {
            func: channels::lastPinTimestamp::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "getSlowmode".to_string(),
        FnMeta {
            func: channels::getSlowmode::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceUserLimit".to_string(),
        FnMeta {
            func: channels::voiceUserLimit::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "createChannel".to_string(),
        FnMeta {
            func: channels::createChannel::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "deleteChannels".to_string(),
        FnMeta {
            func: channels::deleteChannels::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "deleteChannelsByName".to_string(),
        FnMeta {
            func: channels::deleteChannelsByName::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "modifyChannel".to_string(),
        FnMeta {
            func: channels::modifyChannel::run,
            min_args: 1,
            max_args: 6,
        },
    );
    registry.insert(
        "editChannelPerms".to_string(),
        FnMeta {
            func: channels::editChannelPerms::run,
            min_args: 3,
            max_args: 100,
        },
    );
    registry.insert(
        "slowMode".to_string(),
        FnMeta {
            func: channels::slowMode::run,
            min_args: 2,
            max_args: 2,
        },
    );

    // Roles
    registry.insert(
        "roleName".to_string(),
        FnMeta {
            func: roles::roleName::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "roleCount".to_string(),
        FnMeta {
            func: roles::roleCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "roleExists".to_string(),
        FnMeta {
            func: roles::roleExists::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "rolePosition".to_string(),
        FnMeta {
            func: roles::rolePosition::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "roleNames".to_string(),
        FnMeta {
            func: roles::roleNames::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverRoles".to_string(),
        FnMeta {
            func: roles::serverRoles::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "roleColor".to_string(),
        FnMeta {
            func: roles::roleColor::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "rolePerms".to_string(),
        FnMeta {
            func: roles::rolePerms::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "isHoisted".to_string(),
        FnMeta {
            func: roles::isHoisted::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "isMentionable".to_string(),
        FnMeta {
            func: roles::isMentionable::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "hasRole".to_string(),
        FnMeta {
            func: roles::hasRole::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "userRoles".to_string(),
        FnMeta {
            func: roles::userRoles::run,
            min_args: 0,
            max_args: 3,
        },
    );
    registry.insert(
        "highestRole".to_string(),
        FnMeta {
            func: roles::highestRole::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "lowestRole".to_string(),
        FnMeta {
            func: roles::lowestRole::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "highestRoleWithPerms".to_string(),
        FnMeta {
            func: roles::highestRoleWithPerms::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "lowestRoleWithPerms".to_string(),
        FnMeta {
            func: roles::lowestRoleWithPerms::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "roleGrant".to_string(),
        FnMeta {
            func: roles::roleGrant::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "createRole".to_string(),
        FnMeta {
            func: roles::createRole::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "deleteRole".to_string(),
        FnMeta {
            func: roles::deleteRole::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "modifyRole".to_string(),
        FnMeta {
            func: roles::modifyRole::run,
            min_args: 1,
            max_args: 5,
        },
    );
    registry.insert(
        "modifyRolePerms".to_string(),
        FnMeta {
            func: roles::modifyRolePerms::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "colorRole".to_string(),
        FnMeta {
            func: roles::colorRole::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "setUserRoles".to_string(),
        FnMeta {
            func: roles::setUserRoles::run,
            min_args: 2,
            max_args: 100,
        },
    );

    // Bot
    registry.insert(
        "botID".to_string(),
        FnMeta {
            func: bot::botID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "botOwnerID".to_string(),
        FnMeta {
            func: bot::botOwnerID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "botTyping".to_string(),
        FnMeta {
            func: bot::botTyping::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "ping".to_string(),
        FnMeta {
            func: bot::ping::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "uptime".to_string(),
        FnMeta {
            func: bot::uptime::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "executionTime".to_string(),
        FnMeta {
            func: bot::executionTime::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "serverCount".to_string(),
        FnMeta {
            func: bot::serverCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "allMembersCount".to_string(),
        FnMeta {
            func: bot::allMembersCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "serverNames".to_string(),
        FnMeta {
            func: bot::serverNames::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "commandName".to_string(),
        FnMeta {
            func: bot::commandName::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "commandTrigger".to_string(),
        FnMeta {
            func: bot::commandTrigger::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "commandsCount".to_string(),
        FnMeta {
            func: bot::commandsCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "botCommands".to_string(),
        FnMeta {
            func: bot::botCommands::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "slashCommandsCount".to_string(),
        FnMeta {
            func: bot::slashCommandsCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "slashID".to_string(),
        FnMeta {
            func: bot::slashID::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "enabled".to_string(),
        FnMeta {
            func: bot::enabled::run,
            min_args: 1,
            max_args: 2,
        },
    );

    // Servers
    registry.insert(
        "serverName".to_string(),
        FnMeta {
            func: servers::serverName::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverOwner".to_string(),
        FnMeta {
            func: servers::serverOwner::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverIcon".to_string(),
        FnMeta {
            func: servers::serverIcon::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "guildBanner".to_string(),
        FnMeta {
            func: servers::serverBanner::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverDescription".to_string(),
        FnMeta {
            func: servers::serverDescription::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverVerificationLevel".to_string(),
        FnMeta {
            func: servers::serverVerificationLevel::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "membersCount".to_string(),
        FnMeta {
            func: servers::membersCount::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "boostCount".to_string(),
        FnMeta {
            func: servers::boostCount::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "boostLevel".to_string(),
        FnMeta {
            func: servers::boostLevel::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "auditCount".to_string(),
        FnMeta {
            func: audit::auditCount::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntries".to_string(),
        FnMeta {
            func: audit::auditEntries::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditLatest".to_string(),
        FnMeta {
            func: audit::auditLatest::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "auditEntryID".to_string(),
        FnMeta {
            func: audit::auditEntryID::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntryUser".to_string(),
        FnMeta {
            func: audit::auditEntryUser::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntryAction".to_string(),
        FnMeta {
            func: audit::auditEntryAction::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntryTarget".to_string(),
        FnMeta {
            func: audit::auditEntryTarget::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntryReason".to_string(),
        FnMeta {
            func: audit::auditEntryReason::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "auditEntryChanges".to_string(),
        FnMeta {
            func: audit::auditEntryChanges::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "afkTimeout".to_string(),
        FnMeta {
            func: servers::afkTimeout::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "guildExists".to_string(),
        FnMeta {
            func: servers::guildExists::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "serverEmojis".to_string(),
        FnMeta {
            func: servers::serverEmojis::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "serverInvite".to_string(),
        FnMeta {
            func: invites::serverInvite::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "inviteInfo".to_string(),
        FnMeta {
            func: invites::inviteInfo::run,
            min_args: 2,
            max_args: 2,
        },
    );

    // Moderation
    registry.insert(
        "ban".to_string(),
        FnMeta {
            func: moderation::ban::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "kick".to_string(),
        FnMeta {
            func: moderation::kick::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "unban".to_string(),
        FnMeta {
            func: moderation::unban::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "timeout".to_string(),
        FnMeta {
            func: moderation::timeout::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "untimeOut".to_string(),
        FnMeta {
            func: moderation::untimeOut::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "clear".to_string(),
        FnMeta {
            func: moderation::clear::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "getBanReason".to_string(),
        FnMeta {
            func: moderation::getBanReason::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "isBanned".to_string(),
        FnMeta {
            func: moderation::isBanned::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "isTimedOut".to_string(),
        FnMeta {
            func: moderation::isTimedOut::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Voice
    registry.insert(
        "isInVoice".to_string(),
        FnMeta {
            func: voice::isInVoice::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userVoiceChannel".to_string(),
        FnMeta {
            func: voice::userVoiceChannel::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceMembers".to_string(),
        FnMeta {
            func: voice::voiceMembers::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceMemberCount".to_string(),
        FnMeta {
            func: voice::voiceMemberCount::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceOld".to_string(),
        FnMeta {
            func: voice::voiceOld::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "voiceNew".to_string(),
        FnMeta {
            func: voice::voiceNew::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "voiceEmpty".to_string(),
        FnMeta {
            func: voice::voiceEmpty::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceFull".to_string(),
        FnMeta {
            func: voice::voiceFull::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userSelfMuted".to_string(),
        FnMeta {
            func: voice::userSelfMuted::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userSelfDeafened".to_string(),
        FnMeta {
            func: voice::userSelfDeafened::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userStreaming".to_string(),
        FnMeta {
            func: voice::userStreaming::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userServerMuted".to_string(),
        FnMeta {
            func: voice::userServerMuted::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "userServerDeafened".to_string(),
        FnMeta {
            func: voice::userServerDeafened::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "voiceMove".to_string(),
        FnMeta {
            func: voice::voiceMove::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "voiceKick".to_string(),
        FnMeta {
            func: voice::voiceKick::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "mute".to_string(),
        FnMeta {
            func: moderation::mute::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "unmute".to_string(),
        FnMeta {
            func: moderation::unmute::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "deafen".to_string(),
        FnMeta {
            func: moderation::deafen::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "undeafen".to_string(),
        FnMeta {
            func: moderation::undeafen::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // Message
    registry.insert(
        "dm".to_string(),
        FnMeta {
            func: message::dm::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "deleteMessage".to_string(),
        FnMeta {
            func: message::deleteMessage::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "editMessage".to_string(),
        FnMeta {
            func: message::editMessage::run,
            min_args: 3,
            max_args: 3,
        },
    );
    registry.insert(
        "ephemeral".to_string(),
        FnMeta {
            func: message::ephemeral::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "isMentioned".to_string(),
        FnMeta {
            func: message::isMentioned::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "mentioned".to_string(),
        FnMeta {
            func: message::mentioned::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "messageID".to_string(),
        FnMeta {
            func: message::messageID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "repliedMessageID".to_string(),
        FnMeta {
            func: message::repliedMessageID::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "pinMessage".to_string(),
        FnMeta {
            func: message::pinMessage::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "unpinMessage".to_string(),
        FnMeta {
            func: message::unpinMessage::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "publishMessage".to_string(),
        FnMeta {
            func: message::publishMessage::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "getAttachments".to_string(),
        FnMeta {
            func: message::getAttachments::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "getMessage".to_string(),
        FnMeta {
            func: message::getMessage::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "getEmbedData".to_string(),
        FnMeta {
            func: message::getEmbedData::run,
            min_args: 3,
            max_args: 4,
        },
    );
    registry.insert(
        "isMessageEdited".to_string(),
        FnMeta {
            func: message::isMessageEdited::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "useChannel".to_string(),
        FnMeta {
            func: message::useChannel::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // HTTP
    registry.insert(
        "httpGet".to_string(),
        FnMeta {
            func: http::httpGet::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "httpPost".to_string(),
        FnMeta {
            func: http::httpPost::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "httpPut".to_string(),
        FnMeta {
            func: http::httpPut::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "httpDelete".to_string(),
        FnMeta {
            func: http::httpDelete::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "httpPatch".to_string(),
        FnMeta {
            func: http::httpPatch::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "httpAddHeader".to_string(),
        FnMeta {
            func: http::httpAddHeader::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "httpGetHeader".to_string(),
        FnMeta {
            func: http::httpGetHeader::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "httpRemoveHeader".to_string(),
        FnMeta {
            func: http::httpRemoveHeader::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "httpStatus".to_string(),
        FnMeta {
            func: http::httpStatus::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "httpResult".to_string(),
        FnMeta {
            func: http::httpResult::run,
            min_args: 0,
            max_args: 100,
        },
    );

    // JSON
    registry.insert(
        "jsonParse".to_string(),
        FnMeta {
            func: json::jsonParse::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "jsonGet".to_string(),
        FnMeta {
            func: json::jsonGet::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonSet".to_string(),
        FnMeta {
            func: json::jsonSet::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonUnset".to_string(),
        FnMeta {
            func: json::jsonUnset::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonExists".to_string(),
        FnMeta {
            func: json::jsonExists::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonClear".to_string(),
        FnMeta {
            func: json::jsonClear::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "jsonStringify".to_string(),
        FnMeta {
            func: json::jsonStringify::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "jsonPretty".to_string(),
        FnMeta {
            func: json::jsonPretty::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "jsonArray".to_string(),
        FnMeta {
            func: json::jsonArray::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayAppend".to_string(),
        FnMeta {
            func: json::jsonArrayAppend::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayUnshift".to_string(),
        FnMeta {
            func: json::jsonArrayUnshift::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayPop".to_string(),
        FnMeta {
            func: json::jsonArrayPop::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayShift".to_string(),
        FnMeta {
            func: json::jsonArrayShift::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayReverse".to_string(),
        FnMeta {
            func: json::jsonArrayReverse::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArraySort".to_string(),
        FnMeta {
            func: json::jsonArraySort::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayCount".to_string(),
        FnMeta {
            func: json::jsonArrayCount::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonArrayIndex".to_string(),
        FnMeta {
            func: json::jsonArrayIndex::run,
            min_args: 2,
            max_args: 100,
        },
    );
    registry.insert(
        "jsonJoinArray".to_string(),
        FnMeta {
            func: json::jsonJoinArray::run,
            min_args: 2,
            max_args: 100,
        },
    );

    // Components
    registry.insert(
        "addButton".to_string(),
        FnMeta {
            func: components::addButton::run,
            min_args: 2,
            max_args: 6,
        },
    );
    registry.insert(
        "newSelectMenu".to_string(),
        FnMeta {
            func: components::newSelectMenu::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "addSelectMenuOption".to_string(),
        FnMeta {
            func: components::addSelectMenuOption::run,
            min_args: 2,
            max_args: 5,
        },
    );
    registry.insert(
        "newModal".to_string(),
        FnMeta {
            func: components::newModal::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "addTextInput".to_string(),
        FnMeta {
            func: components::addTextInput::run,
            min_args: 2,
            max_args: 7,
        },
    );
    registry.insert(
        "editButton".to_string(),
        FnMeta {
            func: components::editButton::run,
            min_args: 3,
            max_args: 6,
        },
    );
    registry.insert(
        "editSelectMenu".to_string(),
        FnMeta {
            func: components::editSelectMenu::run,
            min_args: 3,
            max_args: 5,
        },
    );
    registry.insert(
        "editSelectMenuOption".to_string(),
        FnMeta {
            func: components::editSelectMenuOption::run,
            min_args: 3,
            max_args: 7,
        },
    );
    registry.insert(
        "removeAllComponents".to_string(),
        FnMeta {
            func: components::removeAllComponents::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "removeButtons".to_string(),
        FnMeta {
            func: components::removeButtons::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "removeComponent".to_string(),
        FnMeta {
            func: components::removeComponent::run,
            min_args: 1,
            max_args: 2,
        },
    );
    registry.insert(
        "defer".to_string(),
        FnMeta {
            func: components::defer::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "inputValue".to_string(),
        FnMeta {
            func: components::inputValue::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "customID".to_string(),
        FnMeta {
            func: components::customID::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "addUserSelect".to_string(),
        FnMeta {
            func: components::addUserSelect::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "addRoleSelect".to_string(),
        FnMeta {
            func: components::addRoleSelect::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "addMentionableSelect".to_string(),
        FnMeta {
            func: components::addMentionableSelect::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "getUserSelectUserID".to_string(),
        FnMeta {
            func: components::getUserSelectUserID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "getUserSelectUserIDs".to_string(),
        FnMeta {
            func: components::getUserSelectUserIDs::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "getUserSelectUserCount".to_string(),
        FnMeta {
            func: components::getUserSelectUserCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "getRoleSelectRoleID".to_string(),
        FnMeta {
            func: components::getRoleSelectRoleID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "getRoleSelectRoleIDs".to_string(),
        FnMeta {
            func: components::getRoleSelectRoleIDs::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "getRoleSelectRoleCount".to_string(),
        FnMeta {
            func: components::getRoleSelectRoleCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "getMentionableSelectUserID".to_string(),
        FnMeta {
            func: components::getMentionableSelectUserID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "getMentionableSelectUserIDs".to_string(),
        FnMeta {
            func: components::getMentionableSelectUserIDs::run,
            min_args: 0,
            max_args: 2,
        },
    );
    registry.insert(
        "getMentionableSelectUserCount".to_string(),
        FnMeta {
            func: components::getMentionableSelectUserCount::run,
            min_args: 0,
            max_args: 0,
        },
    );

    // Loops
    registry.insert(
        "loopIndex".to_string(),
        FnMeta {
            func: loops::loopIndex::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "loopValue".to_string(),
        FnMeta {
            func: loops::loopValue::run,
            min_args: 0,
            max_args: 0,
        },
    );
    // repeat, forSplit, forJson are intercepted in runtime.rs — no registration needed

    // Events
    registry.insert(
        "createEvent".to_string(),
        FnMeta {
            func: events::createEvent::run,
            min_args: 3,
            max_args: 4,
        },
    );
    registry.insert(
        "editEvent".to_string(),
        FnMeta {
            func: events::editEvent::run,
            min_args: 1,
            max_args: 4,
        },
    );
    registry.insert(
        "deleteEvent".to_string(),
        FnMeta {
            func: events::deleteEvent::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventName".to_string(),
        FnMeta {
            func: events::eventName::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventDescription".to_string(),
        FnMeta {
            func: events::eventDescription::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventStart".to_string(),
        FnMeta {
            func: events::eventStart::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventEnd".to_string(),
        FnMeta {
            func: events::eventEnd::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventStatus".to_string(),
        FnMeta {
            func: events::eventStatus::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventChannel".to_string(),
        FnMeta {
            func: events::eventChannel::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventSubscribers".to_string(),
        FnMeta {
            func: events::eventSubscribers::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "eventCount".to_string(),
        FnMeta {
            func: events::eventCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "serverEvents".to_string(),
        FnMeta {
            func: events::serverEvents::run,
            min_args: 0,
            max_args: 0,
        },
    );

    // Stickers
    registry.insert(
        "stickerName".to_string(),
        FnMeta {
            func: stickers::stickerName::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stickerID".to_string(),
        FnMeta {
            func: stickers::stickerID::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stickerExists".to_string(),
        FnMeta {
            func: stickers::stickerExists::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stickerDescription".to_string(),
        FnMeta {
            func: stickers::stickerDescription::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stickerEmoji".to_string(),
        FnMeta {
            func: stickers::stickerEmoji::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stickerCount".to_string(),
        FnMeta {
            func: stickers::stickerCount::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "serverStickers".to_string(),
        FnMeta {
            func: stickers::serverStickers::run,
            min_args: 0,
            max_args: 0,
        },
    );
    registry.insert(
        "deleteSticker".to_string(),
        FnMeta {
            func: stickers::deleteSticker::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // Stage
    registry.insert(
        "stageCreate".to_string(),
        FnMeta {
            func: stage::stageCreate::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "stageEdit".to_string(),
        FnMeta {
            func: stage::stageEdit::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "stageDelete".to_string(),
        FnMeta {
            func: stage::stageDelete::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "stageTopic".to_string(),
        FnMeta {
            func: stage::stageTopic::run,
            min_args: 1,
            max_args: 1,
        },
    );

    // Invites
    registry.insert(
        "createInvite".to_string(),
        FnMeta {
            func: invites::createInvite::run,
            min_args: 1,
            max_args: 3,
        },
    );
    registry.insert(
        "deleteInvite".to_string(),
        FnMeta {
            func: invites::deleteInvite::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "channelInvites".to_string(),
        FnMeta {
            func: invites::channelInvites::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // String extras
    registry.insert(
        "startsWith".to_string(),
        FnMeta {
            func: string::startsWith::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "endsWith".to_string(),
        FnMeta {
            func: string::endsWith::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "indexOf".to_string(),
        FnMeta {
            func: string::indexOf::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "substring".to_string(),
        FnMeta {
            func: string::substring::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "regexMatch".to_string(),
        FnMeta {
            func: string::regexMatch::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "regexReplace".to_string(),
        FnMeta {
            func: string::regexReplace::run,
            min_args: 3,
            max_args: 3,
        },
    );
    registry.insert(
        "padLeft".to_string(),
        FnMeta {
            func: string::padLeft::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "padRight".to_string(),
        FnMeta {
            func: string::padRight::run,
            min_args: 2,
            max_args: 3,
        },
    );

    // Math extras
    registry.insert(
        "pow".to_string(),
        FnMeta {
            func: math::pow::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "abs".to_string(),
        FnMeta {
            func: math::abs::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "round".to_string(),
        FnMeta {
            func: math::round::run,
            min_args: 1,
            max_args: 2,
        },
    );

    // Channel extras
    registry.insert(
        "channelCreated".to_string(),
        FnMeta {
            func: channels::channelCreated::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "channelWebhooks".to_string(),
        FnMeta {
            func: channels::channelWebhooks::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Voice extras
    registry.insert(
        "voiceBitrate".to_string(),
        FnMeta {
            func: voice::voiceBitrate::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Message extras
    registry.insert(
        "messageLink".to_string(),
        FnMeta {
            func: message::messageLink::run,
            min_args: 1,
            max_args: 2,
        },
    );

    // Users extras
    registry.insert(
        "isModerator".to_string(),
        FnMeta {
            func: users::isModerator::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Roles extras
    registry.insert(
        "roleMembers".to_string(),
        FnMeta {
            func: roles::roleMembers::run,
            min_args: 0,
            max_args: 1,
        },
    );
    registry.insert(
        "roleMemberCount".to_string(),
        FnMeta {
            func: roles::roleMemberCount::run,
            min_args: 0,
            max_args: 1,
        },
    );

    // Time extras
    registry.insert(
        "timeDiff".to_string(),
        FnMeta {
            func: time::timeDiff::run,
            min_args: 2,
            max_args: 3,
        },
    );
    registry.insert(
        "fromTimestamp".to_string(),
        FnMeta {
            func: time::fromTimestamp::run,
            min_args: 1,
            max_args: 2,
        },
    );

    // Forums
    registry.insert(
        "createPost".to_string(),
        FnMeta {
            func: forums::createPost::run,
            min_args: 2,
            max_args: 4,
        },
    );
    registry.insert(
        "postTags".to_string(),
        FnMeta {
            func: forums::postTags::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "setPostTags".to_string(),
        FnMeta {
            func: forums::setPostTags::run,
            min_args: 1,
            max_args: 100,
        },
    );
    registry.insert(
        "forumTags".to_string(),
        FnMeta {
            func: forums::forumTags::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "forumTagID".to_string(),
        FnMeta {
            func: forums::forumTagID::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "forumTagEmoji".to_string(),
        FnMeta {
            func: forums::forumTagEmoji::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "forumTagModerated".to_string(),
        FnMeta {
            func: forums::forumTagModerated::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "createForumTag".to_string(),
        FnMeta {
            func: forums::createForumTag::run,
            min_args: 2,
            max_args: 4,
        },
    );
    registry.insert(
        "editForumTag".to_string(),
        FnMeta {
            func: forums::editForumTag::run,
            min_args: 2,
            max_args: 4,
        },
    );
    registry.insert(
        "deleteForumTag".to_string(),
        FnMeta {
            func: forums::deleteForumTag::run,
            min_args: 2,
            max_args: 2,
        },
    );
    registry.insert(
        "forumPosts".to_string(),
        FnMeta {
            func: forums::forumPosts::run,
            min_args: 1,
            max_args: 1,
        },
    );
    registry.insert(
        "forumPostCount".to_string(),
        FnMeta {
            func: forums::forumPostCount::run,
            min_args: 1,
            max_args: 1,
        },
    );
}
