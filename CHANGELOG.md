# Changelog

All notable changes to the ZBR project are documented here and in the [changelog](https://zbrlang.vercel.app/docs/changelog) website.

## v1.5.3 - Fixes and Build Target Optimization

This release fixes critical installation issues for Android/Termux and corrects binary path configurations.

### Fixes
- Fixed duplicate key variable in `postinstall.js` causing Android Termux install to fail
- Fixed `cli.js` binary paths pointing to wrong directory and old binary names
- Removed iOS binary build target

## v1.5.2 - Expansion of Function API and Category Reorganization

This release introduces over 30 new functions, organizes webhook and template functionality into dedicated categories, and adds shard-aware utility functions for advanced bot monitoring.

### Core
- **Shard-Awareness**: Added `shard_id` and `total_shards` to `DiscordContext`, allowing bots to query their sharding status.
- **Webhook Consolidation**: Moved all webhook-related functions (`sendWebhook`, `webhookCreate`, `webhookDelete`) into a new, dedicated `webhooks` category.

### New Functions
- **Bot**: `ZcurrentShard`, `ZtotalShards`
- **Member Management**: `ZmemberPending`, `ZmemberSearch`, `ZmemberFlags`
- **Invites**: `ZcreateInviteWithRoles`
- **Templates**: `ZserverTemplates`, `ZcreateServerTemplate`, `ZsyncServerTemplate`, `ZdeleteServerTemplate`, `ZtemplateName`
- **Onboarding**: `ZonboardingEnabled`, `ZonboardingMode`, `ZonboardingDefaultChannels`, `ZonboardingPrompts`
- **Message**: `ZmessageFlags`, `ZforwardMessage`, `ZmessageSnapshot`, `ZsuppressEmbeds`
- **Permissions**: `ZeffectivePerms`, `ZhasPerm`, `ZpermNames`, `ZchannelOverwrites`
- **Voice & Stage**: `ZvoiceRegion`, `ZvoiceQuality`, `ZstageSpeakers`, `ZstageAudience`
- **Webhooks**: `ZeditWebhookMessage`, `ZdeleteWebhookMessage`, `ZgetWebhookMessage`
- **Utility**: `ZsnowflakeTimestamp`, `ZsnowflakeAge`, `ZbulkBan`

### Modifications
- `ZisTimedOut`: Now supports an optional `returnTimestamp` argument.
- `ZinviteInfo`: Now fetches real data for `uses`, `isTemporary`, and `maxAge`.

### CI/CD
- **Multi-Platform Binary Support**: Updated `.github/workflows/release.yml` and `postinstall.js` to build, name, and correctly download platform-specific binaries (Linux x64/arm64, macOS x64/arm64, Windows x64/arm64).
- **HuggingFace Rebuild Fix**: Updated `release.yml` to force a Dockerfile cache bust using `date +%s` during the HF Space rebuild process, ensuring the latest version is always deployed.

## v1.5.1 - Loader Improvements and Escape Fixes

This release improves the command loader's ability to process complex ZBR scripts and adds new escape sequences.

### Core
- **Multi-line Function Support**: Added brace-depth tracking to the command loader, allowing function calls (e.g., `Zdescription{...}`) to span multiple lines.
- **Newline Preservation**: Fixed an issue where consecutive plain-text lines were incorrectly concatenated without newlines.
- **`\Z` and `\}` Escape Support**: Added `\Z` escape sequence to output literal Z-prefixed text and `\}` to escape curly brackets, correctly handling nested braces in multi-line commands.
- **Brace-Aware Commenting**: Updated brace counting logic so that `//` is only treated as a comment marker when at brace depth 0, preventing URL protocol slashes from breaking multi-line command parsing.
- **Registry Injection**: Updated `loader.rs` to correctly pass the function registry during file loading, enabling bracketless function calls in `.zbr` files.

## v1.5.0 - Parser Flexibility and Alias Robustness

This release introduces major improvements to the parser's flexibility, simplifies function invocation, and resolves critical bugs in the alias system and positional argument handling.

### Core
- **Optional Brackets**: Zero-argument function calls no longer require curly brackets (e.g., `Zusername` works the same as `Zusername{}`).
- **Literal Text Handling**: Unknown Z-prefixed identifiers (e.g., `ZUnknown`) without brackets are now treated as literal text instead of causing runtime errors.
- **Nested Bracketless Calls**: Bracketless function calls are now correctly recognized and evaluated even when nested within the arguments of other function calls (e.g., `ZsendMessage{ZchannelID;hello}`).
- **Alias Fixes**: Resolved argument merging issues where aliased function calls were losing arguments, and added validation to ensure the aliased function is a valid ZBR function.

### Argument Parsing
- **Empty Argument Preservation**: `split_args` now correctly preserves empty arguments between semicolons (e.g., `Zfunc{;arg2}` correctly maps `arg2` to index 1).
- **Positional Argument Robustness**: Refactored 161 functions across the codebase to explicitly handle empty string arguments as "use default" values, preventing positional misalignment bugs.

## v1.4.4 - Runtime Aliases and Reply Fix

This release introduces the `Zalias` system for improved code reusability and fixes a bug in message evaluation.

### Core
- **Runtime Aliases**: Added `Zalias{<expression>;<alias_name>}` which allows users to create runtime aliases for any ZBR expression. Aliases are local to the command execution, support chaining, and include a recursion depth guard to prevent infinite loops.
- **Fixed `Zreply` Evaluation**: Resolved a bug where `Zreply{}` inside a concatenated string (e.g. `Hello Zreply{} World`) would short-circuit evaluation, causing subsequent content to be ignored.
- **Ping Precision**: Updated `Zping{}` to return the actual bot gateway latency with 5 decimal places (in milliseconds) for high-precision monitoring.
- **Execution Time Precision**: Updated `ZexecutionTime{}` to return command execution time in milliseconds with 5 decimal places.

## v1.4.3 - Performance and Error Handling Improvements

This release focuses on improving bot performance and robustness by caching execution data and surfacing critical database errors.

### Core
- **AST Caching**: Pre-parses ZBR scripts into an AST on load, eliminating redundant parsing on every execution.
- **Variable Caching**: Implemented a per-execution variable cache in `DiscordContext` to eliminate redundant database reads for the same variable within a single command execution.
- **Regex Optimization**: Moved trigger regex compilation to a `static` initializer, eliminating repeated compilation on every incoming message.

### Error Handling
- **Structured Error Reporting**: Refactored command loading to produce detailed, context-aware error messages (file, line number) when command files fail to parse.
- **Database Error Propagation**: Database write failures are no longer silently ignored and now surface as error messages in Discord.
- **Standardized Error Messages**: Aligned database failure messages with the centralized `error_messages.rs` formatting system (`action_failed_reason`).

## v1.4.2 - Mobile support and version fix

### Distribution
- **Android support**: Added `aarch64-linux-android` binary target; ZBR now runs on Android via Termux.
- **iOS support**: Added `aarch64-apple-ios` binary target; ZBR now runs on iOS via iSH.

### CLI
- **Fixed `zbr version`**: Now reports the actual binary version instead of the stale npm package version.
- **Fixed `zbr update`**: Version is now always accurate after updating.

## v1.4.1 - Parser fix and Zupdate interaction

This release addresses a critical parser bug and introduces a new interaction feature for better message management in Discord.

### Core
- **Fixed Parser Data Loss**: Resolved a bug in `parse_arg` where arguments containing multiple function calls (e.g., inside `Zif`) were being truncated.

### New Functions
- `Zupdate{}`: Signals that an interaction (button/select menu) should update the original message instead of sending a new reply.
- `ZspliceText{text;start;length;replacement}`: Modifies a string by removing a specified number of characters and inserting new text.

## v1.4.0 - Centralized error handling across all functions

All 390+ ZBR functions now share a single centralized error system in `src/error_messages.rs`. Every `FnOutput::error()` call uses the same formatting helpers, producing consistent `Line N: Zfunction - message` output everywhere.

### Core
- **26 centralized error helpers**: `too_few_args`, `expected_snowflake`, `expected_url`, `out_of_range`, `not_found`, `action_failed_reason`, `not_available`, and more, all producing uniform error messages.
- **Every function updated**: All 394+ `FnOutput::error` calls across the codebase now route through `crate::error_messages::*`.
- **Helper modules aligned**: `math/helpers.rs` (`parse_f64`/`parse_i64`), `permissions/helpers.rs`, `audit/helpers.rs`, and `json/helpers.rs` all use the centralized system.
- `src/error_messages.rs` added: single source of truth for error messaging.

## v1.3.0 - In-process engine, automod, polls, soundboard, and utility functions

This release eliminates the HTTP runtime server, running the engine in-process for lower latency and simpler deployment. It also adds 5 new function categories and over 30 new functions.

### Core
- **Removed HTTP runtime server**: The Axum-based HTTP server (`/run` endpoint) is gone. Code execution now happens directly in-process via the new `executor` module, eliminating network overhead and the need for `axum`, `tower`, and `tower-http` dependencies.
- **SSRF Protection**: HTTP functions now validate URLs against a blocklist of private/reserved IPs and known dangerous hostnames before executing requests.
- **Header Validation**: `ZhttpAddHeader` blocks dangerous headers (cookie, host, connection, transfer-encoding, etc.) for security.
- **Auto Content-Type**: HTTP requests with JSON-like bodies now auto-detect and set `Content-Type: application/json` when no content type header is present.
- `ZstartThread` now accepts a 6th `private` argument to create private or public threads.
- Commented-out `ZonlyForIDs` guard added to the eval example command.
- Added `url`, `base64`, `sha2`, `md-5` dependencies.

### New function categories

**Automod**
`ZautomodRule`, `ZautomodRuleCreate`, `ZautomodRuleEdit`, `ZautomodRuleDelete`, `ZautomodRules`
Alias: `ZautomodRuleUpdate` → `ZautomodRuleEdit`

**Application Emojis**
`ZappEmojis`, `ZappEmojiCreate`, `ZappEmojiDelete`

**Polls**
`ZpollAddAnswer`, `ZpollAllowMultiselect`, `ZpollAnswerVoters`, `ZpollAnswerVotes`, `ZpollCreate`, `ZpollEnd`, `ZpollGet`, `ZpollSend`

**Soundboard**
`ZsoundboardCreate`, `ZsoundboardDefaultSounds`, `ZsoundboardDelete`, `ZsoundboardEdit`, `ZsoundboardPlay`, `ZsoundboardSound`, `ZsoundboardSounds`

**Utility**
`Zbase64Decode`, `Zbase64Encode`, `Zduration`, `Zentitlements`, `Zmd5`, `Zsha256`, `Zskus`

### New individual functions

**Channel**: `ZsyncPerms`
**Forum**: `ZforumPostLock`, `ZforumPostPin`
**Message**: `ZpinList`
**Server**: `ZserverLockdown`, `ZserverModify`, `ZeditWelcomeScreen`, `ZwelcomeScreen`
**Sticker**: `ZstickerCreate`, `ZstickerEdit`
**Thread**: `ZthreadArchive`, `ZthreadList`, `ZthreadMetadata`, `ZthreadPin`, `ZthreadUnarchive`
**Voice**: `ZvoiceRequestToSpeak`, `ZvoiceStatus`, `ZvoiceSuppress`

## v1.2.0 - Bot owner fix, server/thread/voice functions, and CLI improvements

This release fixes bot owner resolution for team/group bots, adds new server/thread/voice functions, improves CLI command support, and expands project initialization.

### Core
- Fixed `botOwnerID` so it returns the actual bot owner instead of the team ID when the bot is part of a team/group.
- Added `version`, `list`, and `new <type>` CLI commands.
- Updated `init` to support creating a project in a new folder via `init <folder>`.
- Removed archived bot voice helper code from `archive/bot-voice`.

### Functions added

**Server functions**
- `serverChannels`, `serverRoles`

**Thread functions**
- `threadArchived`, `threadLocked`, `threadParentID`

**Voice functions**
- `voiceEmpty`, `voiceFull`, `voiceNew`, `voiceOld`

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

## v1.0.0 - Production Release

End of Alpha and the first stable production release. This version introduces the official ZBR CLI, automated installation, and multi-OS support.

### CLI & Distribution
- **New ZBR CLI**: The entire engine is now managed via a unified global command: `zbr`.
- **Project Initialization**: `zbr init` instantly bootstraps a new project with a recommended folder structure, configuration files, and example scripts.
- **Unified Runner**: `zbr run` launches the high-performance Rust execution engine and starts your bot.
- **Multi-OS Support**: Official support and pre-built binaries for **Linux (x64)**, **macOS (x64 & ARM64)**, and **Windows (x64)**.
- **Smart Installation**: Distributed via npm with a tiny footprint; the CLI automatically downloads the correct binary for your system on install.

### Features
- Includes all features and functions from **Alpha v5** and earlier. Scroll down to see the full history from Alpha v1 to v5.

## Alpha v5

Loop system, async execution, full voice channel coverage, scheduled events, forum channels, stage channels, stickers, invite management, regex, extended string and math utilities, and more.

### Core
- **Loop system**: `Zrepeat{N;code}` runs a code block N times (max 1000); `ZforSplit{code}` iterates over the current split text; `ZforJson{key;...;code}` iterates over a JSON array at a key path. All three are lazy-evaluated. `ZloopIndex{}` and `ZloopValue{}` expose the current iteration state inside any loop body
- **Async execution**: `Zasync{name;code}` spawns a named background task that runs the code block concurrently; `Zawait{name}` blocks until that task completes and returns its result
- **Deferred execution**: `Zdelay{duration;code}` runs a code block after a delay (e.g. `10s`, `2m`) in a background task; `ZreplyIn{duration;content}` replies to the trigger message after a delay. Both are fire-and-forget and cancelled on restart

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

## Alpha v4

Moderation, message operations, HTTP requests, JSON manipulation, full control flow, error handling, and the component/interaction system.

### Core
- `#type interaction`: new command type for component interaction handlers
- `#type event`: new command type for Discord gateway event handlers
- `onInteraction{id?}` trigger: runs when a button, select menu, or modal is submitted. Specific handler (`onInteraction{my_button}`) takes priority over catch-all (`onInteraction`)
- `ZcustomID{}`: returns the custom_id of the current interaction
- `ZinputValue{fieldID}`: reads a submitted modal text input field
- `Zdefer{}`: defers an interaction response
- `ZsuppressErrors{text?;embedIndex?}`: suppress errors and show a custom message or embed instead
- `ZtryRun{code;fallback?}`: lazy try/catch
- `Zstop{}`: halt execution silently
- `Zerror{message}`: halt with a custom error message
- `ZonlyIf{condition;error}`: guard function
- `ZargsCheck{min;max?;error}`: validate argument count
- `Znot{value}`: flip a boolean
- `ZisBoolean`, `ZisInteger`, `ZisNumber`, `ZisSlash`, `ZisValidHex`: type check functions
- `ZhttpAddHeader{}`: add headers for subsequent HTTP requests
- `ZhttpStatus{}`, `ZhttpResult{key;...}`: read HTTP response status and JSON body
- Full JSON object system with mutable state per execution
- `Zurl{mode;text}`: URL encode/decode
- `ZbyteCount`, `ZargCount`, `ZcheckContains`, `ZremoveLinks`: new string utilities
- `ZgetTimestamp{unit?}`: Unix timestamp
- `ZuserJoined{userID?;format?}`: guild join date
- `ZvarExistError{name;error}`: halt if global variable doesn't exist
- `ZonlyIfMessageContains{message;word;...;error}`: guard by message content

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

## Alpha v3

User, role, channel, server, and bot functions. Full Discord entity coverage.

### Core
- Context functions: `Zmessage`, `Zoption`, `ZuserID`, `ZchannelID`, `ZguildID`, `ZroleID`, `Zusername`
- `Zenabled{}`: enable/disable a command at runtime
- `ZallowUserMentions{}`, `ZallowRoleMentions{}`: control which mentions the bot pings
- `Zephemeral{}`: make slash command responses ephemeral
- `ZuseChannel{}`: redirect bot output to a different channel

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

## Alpha v2

Reactions, emojis, text splitting, permissions, threads, and blacklists. Introduced the `Zif` condition system.

### Core
- `Zif{condition;then;else?}`: lazy conditional evaluation with `==`, `!=`, `>`, `<`, `>=`, `<=`, `contains`, `startsWith`, `endsWith`, `&&`, `||` operators
- `ZcheckCondition{}`: evaluate a condition string and return `true`/`false`

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
- HTTP runtime server (Axum): bot sends context, runtime evaluates and returns response
- SQLite persistence via sqlx for variables and cooldowns
- `Zeval{}`: evaluates ZBR code dynamically at runtime
- `Zreply{}`: makes the bot reply to the triggering message

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