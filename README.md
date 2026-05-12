# ZBR

<p align="left">
  <img width="830" height="240" src="assets/images/ZBR_banner.png" alt="ZBR Banner" />
</p>

ZBR is a scripting language for Discord bots. You write commands as plain `.zbr` files using ZBR functions — no boilerplate, no event handlers, no framework knowledge required. Drop a file in the `commands/` folder and it runs.

---

## What it looks like

```
#trigger !rank
#name Rank Command
#type prefix

Zvar{xp;ZgetUserVar{xp}}
Zvar{level;ZgetUserVar{level}}

Ztitle{Zusername{}'s Rank}
Zdescription{Level: Zvar{level}
XP: Zvar{xp}}
Zcolor{#5865F2}
ZaddField{Server Rank;#Zvar{rank};true}
```

```
#trigger /ban
#name Ban Command
#type slash
#description Ban a user from the server
#option user|User to ban|user|required
#option reason|Reason for the ban|string|optional

ZonlyIf{ZisAdmin{}==true;You need administrator permission to use this command}
Zban{Zoption{user};Zoption{reason}}
Banned Zoption{user}.
```

```
#trigger onInteraction{confirm_ban}
#name Confirm Ban Handler
#type interaction

Zephemeral{}
Zban{ZgetServerVar{pending_ban}}
Done.
```

---

## How it works

ZBR runs as a local HTTP server alongside your Discord bot. When a command is triggered, the bot sends the execution context to the ZBR runtime, which evaluates the script and returns the response. The bot then sends that response to Discord.

Commands are plain text files. Each file has a header section (lines starting with `#`) that defines the trigger, name, and type, followed by the ZBR code that runs when the command is invoked.

---

## Command types

| Type | Trigger format | When it runs |
|---|---|---|
| `prefix` | `!command` | When a message starts with the trigger |
| `slash` | `/command` | When a slash command is invoked |
| `interaction` | `onInteraction{id?}` | When a button, select menu, or modal is submitted |
| `event` | `onMessage`, `onMemberJoin`, etc. | When a Discord gateway event fires |

---

## Function syntax

Functions are called with a `Z` prefix, followed by the function name and arguments in `{}` separated by `;`.

```
ZfunctionName{arg1;arg2;arg3}
```

Functions can be nested — the inner call evaluates first:

```
Zsum{ZgetUserVar{xp};100}
```

Plain text on a line is output as-is. Functions and text can be mixed:

```
You have Zsum{ZgetUserVar{xp};0} XP.
```

Lines starting with `//` are comments.

---

## What's included

- **Math** — `Zsum`, `Zsub`, `Zdiv`, `Zmulti`, `Zcalculate`, `Zrandom`, `Zsort`, and more
- **String** — `Zlowercase`, `ZreplaceText`, `ZcropText`, `ZcheckContains`, `Zurl`, and more
- **Embeds** — full embed builder with multi-embed support and `ZsendEmbed`
- **Variables** — user, server, channel, global, and temp scopes persisted in SQLite
- **Cooldowns** — per-user, per-server, and global cooldowns
- **Moderation** — `Zban`, `Zkick`, `Zunban`, `Ztimeout`, `Zclear`, and more
- **Roles & channels** — create, modify, delete, query roles and channels
- **Components** — buttons, select menus (string, user, role, mentionable), modals
- **HTTP** — `ZhttpGet`, `ZhttpPost`, `ZhttpPut`, `ZhttpDelete`, `ZhttpPatch` with header support and JSON navigation via `ZhttpResult`
- **JSON** — full mutable JSON object with `ZjsonParse`, `ZjsonSet`, `ZjsonGet`, array operations, and more
- **Control flow** — `Zif` with `==`, `!=`, `>`, `<`, `>=`, `<=`, `contains`, `startsWith`, `endsWith`, `&&`, `||`
- **Error handling** — `ZtryRun`, `ZsuppressErrors`, `Zstop`, `Zerror`
- **Interactions** — `onInteraction{id?}` handlers for buttons, select menus, and modals with `ZcustomID`, `ZinputValue`, and select value getters

---

## Example commands

The `commands/` folder contains ready-to-run examples:

| File | Type | What it does |
|---|---|---|
| `ping.zbr` | slash | Replies with pong and echoes your input |
| `hello.zbr` | prefix | Greets the user with their name, channel, and server |
| `rank.zbr` | prefix | Reads user variables and sends a rank embed |
| `eval.zbr` | prefix | Evaluates ZBR code on the fly |

---

## Built with

- [Rust](https://www.rust-lang.org/)
- [Serenity](https://github.com/serenity-rs/serenity) — Discord API
- [Axum](https://github.com/tokio-rs/axum) — HTTP runtime server
- [SQLite](https://www.sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx) — variable and cooldown persistence

---

## License

MIT — see [LICENSE](LICENSE)
