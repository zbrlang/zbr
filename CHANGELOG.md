# ZBR Changelog

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
