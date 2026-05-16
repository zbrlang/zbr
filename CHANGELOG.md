# Changelog

All notable changes to the ZBR project are documented here and in the [changelog](https://zbr-website.vercel.app/docs/changelog) website.

---

## v1.1.0 - Audit & Event Additions

This release adds a new audit-log function category and several gateway-based event triggers.

### Audit
- Added a new `audit` function category exposing functions:
	- `ZauditCount`, `ZauditEntries`, `ZauditLatest`, `ZauditEntryID`, `ZauditEntryUser`, `ZauditEntryAction`, `ZauditEntryTarget`, `ZauditEntryReason`, `ZauditEntryChanges`
- Functions fetch and return guild audit log data via the Discord API (JSON output for structured fields).

### Events
- Added new triggers: `onBotJoin`, `onBotLeave`, `onBoostAdd`, `onBoostRemove`.
- `onBotJoin` and `onBotLeave` map to runtime guild join/leave events and fire only for guilds the bot joins or leaves while online.
- Boost event detection implemented via guild update comparisons of `premium_subscription_count`.

---

## v1.0.0 - Production Release

End of Alpha and the first stable production release. This version introduces the official ZBR CLI, automated installation, and multi-OS support.

### CLI & Distribution
- **New ZBR CLI** — The entire engine is now managed via a unified global command: `zbr`.
- **Project Initialization** — `zbr init` instantly bootstraps a new project with a recommended folder structure, configuration files, and example scripts.
- **Unified Runner** — `zbr run` launches the high-performance Rust execution engine and starts your bot.
- **Multi-OS Support** — Official support and pre-built binaries for **Linux (x64)**, **macOS (x64 & ARM64)**, and **Windows (x64)**.
- **Smart Installation** — Distributed via npm with a tiny footprint; the CLI automatically downloads the correct binary for your system on install.

### Features
- Includes all features and functions from **Alpha v5** and earlier. Scroll down to see the full history from Alpha v1 to v5.

---

## Alpha v5

Loop system, async execution, full voice channel coverage, scheduled events, forum channels, stage channels, stickers, invite management, regex, extended string and math utilities, and more.

### Core
- **Loop system** — `Zrepeat{N;code}` runs a code block N times (max 1000); `ZforSplit{code}` iterates over the current split text; `ZforJson{key;...;code}` iterates over a JSON array at a key path. All three are lazy-evaluated. `ZloopIndex{}` and `ZloopValue{}` expose the current iteration state inside any loop body
- **Async execution** — `Zasync{name;code}` spawns a named background task that runs the code block concurrently; `Zawait{name}` blocks until that task completes and returns its result
- **Deferred execution** — `Zdelay{duration;code}` runs a code block after a delay (e.g. `10s`, `2m`) in a background task; `ZreplyIn{duration;content}` replies to the trigger message after a delay. Both are fire-and-forget and cancelled on restart

### Functions added

**Moderation functions**
`Zmute`, `Zunmute`, `Zdeafen`, `Zundeafen`

**Voice functions**
`ZisInVoice`, `ZuserVoiceChannel`, `ZuserStreaming`, `ZuserSelfDeafened`, `ZuserSelfMuted`, `ZuserServerDeafened`, `ZuserServerMuted`, `ZvoiceKick`, `ZvoiceMove`, `ZvoiceMembers`, `ZvoiceMemberCount`, `ZvoiceBitrate`

**Math functions**
`Zabs`, `Zpow`, `Zround`

**String functions**
`ZstartsWith`, `ZendsWith`, `ZindexOf`, `Zsubstring`, `ZpadLeft`, `ZpadRight`, `ZregexMatch`, `ZregexReplace`

**Time functions**
`ZfromTimestamp`, `ZtimeDiff`

**Channel functions**
`ZchannelCreated`, `ZchannelWebhooks`, `ZchannelInvites`

**Invite functions**
`ZcreateInvite`, `ZdeleteInvite`

**Role functions**
`ZroleMemberCount`, `ZroleMembers`

**User functions**
`ZisModerator`

**Message functions**
`ZmessageLink`

**HTTP functions**
`ZhttpGetHeader`, `ZhttpRemoveHeader`

**Stage functions**
`ZstageCreate`, `ZstageEdit`, `ZstageDelete`, `ZstageTopic`

**Sticker functions**
`ZserverStickers`, `ZstickerCount`, `ZstickerName`, `ZstickerID`, `ZstickerExists`, `ZstickerDescription`, `ZstickerEmoji`, `ZdeleteSticker`

**Event functions**
`ZserverEvents`, `ZeventCount`, `ZcreateEvent`, `ZeditEvent`, `ZdeleteEvent`, `ZeventName`, `ZeventDescription`, `ZeventStart`, `ZeventEnd`, `ZeventStatus`, `ZeventChannel`, `ZeventSubscribers`

**Forum functions**
`ZforumTags`, `ZforumTagID`, `ZforumTagEmoji`, `ZforumTagModerated`, `ZcreateForumTag`, `ZeditForumTag`, `ZdeleteForumTag`, `ZforumPosts`, `ZforumPostCount`, `ZcreatePost`, `ZpostTags`, `ZsetPostTags`

---

## Alpha v4

Moderation, message operations, HTTP requests, JSON manipulation, full control flow, error handling, and the component/interaction system.

### Core
- `#type interaction` — new command type for component interaction handlers
- `#type event` — new command type for Discord gateway event handlers
- `onInteraction{id?}` trigger — runs when a button, select menu, or modal is submitted. Specific handler (`onInteraction{my_button}`) takes priority over catch-all (`onInteraction`)
- `ZcustomID{}` — returns the custom_id of the current interaction
- `ZinputValue{fieldID}` — reads a submitted modal text input field
- `Zdefer{}` — defers an interaction response
- `ZsuppressErrors{text?;embedIndex?}` — suppress errors and show a custom message or embed instead
- `ZtryRun{code;fallback?}` — lazy try/catch
- `Zstop{}` — halt execution silently
- `Zerror{message}` — halt with a custom error message
- `ZonlyIf{condition;error}` — guard function
- `ZargsCheck{min;max?;error}` — validate argument count
- `Znot{value}` — flip a boolean
- `ZisBoolean`, `ZisInteger`, `ZisNumber`, `ZisSlash`, `ZisValidHex` — type check functions
- `ZhttpAddHeader{}` — add headers for subsequent HTTP requests
- `ZhttpStatus{}`, `ZhttpResult{key;...}` — read HTTP response status and JSON body
- Full JSON object system with mutable state per execution
- `Zurl{mode;text}` — URL encode/decode
- `ZbyteCount`, `ZargCount`, `ZcheckContains`, `ZremoveLinks` — new string utilities
- `ZgetTimestamp{unit?}` — Unix timestamp
- `ZuserJoined{userID?;format?}` — guild join date
- `ZvarExistError{name;error}` — halt if global variable doesn't exist
- `ZonlyIfMessageContains{message;word;...;error}` — guard by message content

### Functions added

**Moderation functions**
`Zban`, `Zkick`, `Zunban`, `Ztimeout`, `ZuntimeOut`, `Zclear`, `ZgetBanReason`, `ZisBanned`, `ZisTimedOut`

**Message functions**
`Zdm`, `ZdeleteMessage`, `ZeditMessage`, `Zephemeral`, `ZisMentioned`, `Zmentioned`, `ZmessageID`, `ZrepliedMessageID`, `ZpinMessage`, `ZunpinMessage`, `ZpublishMessage`, `ZgetAttachments`, `ZgetMessage`, `ZgetEmbedData`, `ZisMessageEdited`, `ZuseChannel`

**HTTP functions**
`ZhttpGet`, `ZhttpPost`, `ZhttpPut`, `ZhttpDelete`, `ZhttpPatch`, `ZhttpAddHeader`, `ZhttpStatus`, `ZhttpResult`

**JSON functions**
`ZjsonParse`, `ZjsonGet`, `ZjsonSet`, `ZjsonUnset`, `ZjsonExists`, `ZjsonClear`, `ZjsonStringify`, `ZjsonPretty`, `ZjsonArray`, `ZjsonArrayAppend`, `ZjsonArrayUnshift`, `ZjsonArrayPop`, `ZjsonArrayShift`, `ZjsonArrayReverse`, `ZjsonArraySort`, `ZjsonArrayCount`, `ZjsonArrayIndex`, `ZjsonJoinArray`

**Control functions**
`Zif`, `Znot`, `ZcheckCondition`, `ZonlyIf`, `ZargsCheck`, `ZisBoolean`, `ZisInteger`, `ZisNumber`, `ZisSlash`, `ZisValidHex`

**Error functions**
`Zstop`, `Zerror`, `ZsuppressErrors`, `ZtryRun`

**Component functions**
`ZaddButton`, `ZnewSelectMenu`, `ZaddSelectMenuOption`, `ZaddUserSelect`, `ZaddRoleSelect`, `ZaddMentionableSelect`, `ZnewModal`, `ZaddTextInput`, `ZeditButton`, `ZeditSelectMenu`, `ZeditSelectMenuOption`, `ZremoveAllComponents`, `ZremoveButtons`, `ZremoveComponent`, `Zdefer`, `ZinputValue`, `ZcustomID`, `ZgetUserSelectUserID`, `ZgetUserSelectUserIDs`, `ZgetUserSelectUserCount`, `ZgetRoleSelectRoleID`, `ZgetRoleSelectRoleIDs`, `ZgetRoleSelectRoleCount`, `ZgetMentionableSelectUserID`, `ZgetMentionableSelectUserIDs`, `ZgetMentionableSelectUserCount`

---

## Alpha v3

User, role, channel, server, and bot functions. Full Discord entity coverage.

### Core
- Context functions: `Zmessage`, `Zoption`, `ZuserID`, `ZchannelID`, `ZguildID`, `ZroleID`, `Zusername`
- `Zenabled{}` — enable/disable a command at runtime
- `ZallowUserMentions{}`, `ZallowRoleMentions{}` — control which mentions the bot pings
- `Zephemeral{}` — make slash command responses ephemeral
- `ZuseChannel{}` — redirect bot output to a different channel

### Functions added

**User functions**
`ZuserAvatar`, `ZuserServerAvatar`, `ZuserBadge`, `ZuserBanner`, `ZuserBannerColor`, `ZuserExists`, `ZuserPerms`, `ZchangeNickname`, `ZdisplayName`, `ZuserStatus`, `ZisAdmin`, `ZisBooster`, `ZisBot`, `ZisUserDMEnabled`, `ZuserJoined`, `ZcreationDate`

**Role functions**
`ZroleColor`, `ZroleCount`, `ZroleExists`, `ZroleGrant`, `ZroleID`, `ZroleName`, `ZroleNames`, `ZrolePerms`, `ZrolePosition`, `ZhasRole`, `ZhighestRole`, `ZlowestRole`, `ZhighestRoleWithPerms`, `ZlowestRoleWithPerms`, `ZuserRoles`, `ZcreateRole`, `ZdeleteRole`, `ZmodifyRole`, `ZmodifyRolePerms`, `ZcolorRole`, `ZsetUserRoles`, `ZisHoisted`, `ZisMentionable`

**Channel functions**
`ZchannelCount`, `ZchannelExists`, `ZchannelName`, `ZchannelNames`, `ZchannelPosition`, `ZchannelTopic`, `ZchannelType`, `ZcategoryChannels`, `ZcategoryCount`, `ZcategoryID`, `ZafkChannelID`, `ZrulesChannelID`, `ZsystemChannelID`, `ZdmChannelID`, `ZparentID`, `ZisNSFW`, `ZlastMessageID`, `ZlastPinTimestamp`, `ZgetSlowmode`, `ZslowMode`, `ZvoiceUserLimit`, `ZcreateChannel`, `ZdeleteChannels`, `ZdeleteChannelsByName`, `ZmodifyChannel`, `ZeditChannelPerms`

**Server functions**
`ZserverName`, `ZserverOwner`, `ZserverIcon`, `ZguildBanner`, `ZserverDescription`, `ZserverVerificationLevel`, `ZmembersCount`, `ZboostCount`, `ZboostLevel`, `ZafkTimeout`, `ZguildExists`, `ZserverEmojis`, `ZserverInvite`, `ZinviteInfo`

**Bot functions**
`ZbotID`, `ZbotOwnerID`, `ZbotTyping`, `Zping`, `Zuptime`, `ZexecutionTime`, `ZserverCount`, `ZallMembersCount`, `ZserverNames`, `ZcommandName`, `ZcommandTrigger`, `ZcommandsCount`, `ZbotCommands`, `ZslashCommandsCount`, `ZslashID`, `Zenabled`

---

## Alpha v2

Reactions, emojis, text splitting, permissions, threads, and blacklists. Introduced the `Zif` condition system.

### Core
- `Zif{condition;then;else?}` — lazy conditional evaluation with `==`, `!=`, `>`, `<`, `>=`, `<=`, `contains`, `startsWith`, `endsWith`, `&&`, `||` operators
- `ZcheckCondition{}` — evaluate a condition string and return `true`/`false`

### Functions added

**Reaction functions**
`ZaddReactions`, `ZaddCmdReactions`, `ZaddMessageReactions`, `ZgetReactions`, `ZclearReactions`, `ZuserReacted`

**Emoji functions**
`ZaddEmoji`, `ZcustomEmoji`, `ZemojiExists`, `ZemojiName`, `ZemoteCount`, `ZisEmojiAnimated`, `ZremoveEmoji`

**Text split functions**
`ZtextSplit`, `ZsplitText`, `ZgetTextSplitIndex`, `ZgetTextSplitLength`, `ZjoinSplitText`, `ZremoveSplitTextElement`, `ZeditSplitText`

**Permission functions**
`ZcheckUserPerms`, `ZignoreChannels`, `ZonlyAdmin`, `ZonlyBotChannelPerms`, `ZonlyBotPerms`, `ZonlyForCategories`, `ZonlyForChannels`, `ZonlyForIDs`, `ZonlyForRoles`, `ZonlyForRoleIDs`, `ZonlyForServers`, `ZonlyForUsers`, `ZonlyNSFW`, `ZonlyPerms`

**Thread functions**
`ZstartThread`, `ZeditThread`, `ZthreadAddMember`, `ZthreadRemoveMember`, `ZthreadMessageCount`, `ZthreadUserCount`

**Blacklist functions**
`ZblackListIDs`, `ZblackListUsers`, `ZblackListRoles`, `ZblackListRolesIDs`, `ZblackListServers`

---

## Alpha v1

Initial release. Established the core runtime, parser, and execution model.

### Core
- ZBR scripting language runtime built in Rust
- Line-by-line execution with `Z`-prefixed function call syntax
- Argument parsing with `;` separator, nested function calls, escape sequences (`\{`, `\;`, `\\`)
- `#trigger`, `#name`, `#type`, `#description`, `#scope`, `#option` command header system
- Prefix command support (`#type prefix`)
- Slash command support (`#type slash`) with typed options
- Hot-reload: `commands/` folder is watched and reloaded on file change
- HTTP runtime server (Axum) — bot sends context, runtime evaluates and returns response
- SQLite persistence via sqlx for variables and cooldowns
- `Zeval{}` — evaluates ZBR code dynamically at runtime
- `Zreply{}` — makes the bot reply to the triggering message

### Functions added

**Embed functions**
`Ztitle`, `ZtitleURL`, `Zdescription`, `Zcolor`, `Zauthor`, `ZauthorIcon`, `ZauthorURL`, `Zfooter`, `ZfooterIcon`, `Zthumbnail`, `Zimage`, `Ztimestamp`, `ZaddField`, `ZsendEmbed`, `ZsendMessage`, `ZwebhookCreate`, `ZwebhookDelete`, `ZsendWebhook`

**Math functions**
`Zsum`, `Zsub`, `Zdiv`, `Zmulti`, `Zcalculate`, `Zceil`, `Zfloor`, `Zmax`, `Zmin`, `Zmodulo`, `Zsqrt`, `Zsort`, `Zrandom`

**Time functions**
`Ztime`, `Zdate`, `Zday`, `Zmonth`, `Zyear`, `Zhour`, `Zminute`, `Zsecond`

**Variable functions**
`ZgetUserVar`, `ZsetUserVar`, `ZresetUserVar`, `ZgetServerVar`, `ZsetServerVar`, `ZresetServerVar`, `ZgetChannelVar`, `ZsetChannelVar`, `ZresetChannelVar`, `ZgetVar`, `ZsetVar`, `Zvar`, `ZlistVar`, `ZvarExists`

**String functions**
`Zlowercase`, `Zuppercase`, `ZcharCount`, `ZlinesCount`, `ZnumberSeparator`, `ZcropText`, `ZreplaceText`, `Ztitlecase`, `ZtrimContent`, `ZtrimSpace`

**Random functions**
`ZrandomCategoryID`, `ZrandomChannelID`, `ZrandomGuildID`, `ZrandomMention`, `ZrandomRoleID`, `ZrandomString`, `ZrandomText`, `ZrandomUser`, `ZrandomUserID`

**Cooldown functions**
`Zcooldown`, `ZserverCooldown`, `ZglobalCooldown`, `ZgetCooldown`, `ZchangeCooldownTime`
