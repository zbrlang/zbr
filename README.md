# ZBR

ZBR is a scripting language for Discord bots powered by a high-performance Rust runtime engine. 
You write commands as plain `.zbr` files using ZBR functions, no boilerplate, no event handlers, no framework knowledge required.

## Quick Start

Get your bot up and running in seconds:

### Option 1: Webapp Editor (Recommended)
Head over to the [ZBR Webapp](https://zbr-webapp.vercel.app) for a zero-setup development experience.
1. Build your commands, manage variables, and tweak settings visually.
2. Press **Run** to launch your bot instantly in the cloud for testing.
3. Your bot is now live for testing!

> **Note:** The Webapp "Run" button is strictly for rapid prototyping and interactive testing. For 24/7 production hosting, you must export your project as a ZIP and use the CLI.

### Option 2: CLI
1. **Install the CLI globally:**
   ```bash
   npm i @zbrlang/zbr
   ```

2. **Initialize a new project:**
   ```bash
   zbr init my-bot
   ```
   This creates your `commands/` folder, `zbr.json` config, and a `.env` file inside of **my-bot**.

3. **Navigate to your project directory**
   ```bash
   cd my-bot
   ```

4. **Configure your token:**
   Open the `.env` file and paste your Discord Bot Token.

5. **Run the engine:**
   ```bash
   zbr run
   ```
   Your bot is now live and loading scripts from the `commands/` directory!

## Webapp Editor

For those who prefer a visual development environment, the [ZBR Webapp](https://zbr-webapp.vercel.app) allows you to build and test your bot entirely in your browser.

- **Instant Prototyping:** Write code and launch your bot with a single click using the built-in cloud runner, no local setup required.
- **Visual Interface:** Easily design your bot logic, manage variables, and configure settings without touching a terminal.
- **Hybrid Workflow:** Use the Webapp for rapid development and testing, then export your project as a ZIP to host it permanently on your own server using the ZBR CLI.

## What it looks like

```
#trigger !rank
#name Rank Command
#type prefix

Zvar{xp;ZgetUserVar{xp}}
Zvar{level;ZgetUserVar{level}}

Ztitle{Zusername's Rank}
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

ZonlyIf{ZisAdmin==true;You need administrator permission to use this command}
Zban{Zoption{user};Zoption{reason}}
Banned Zoption{user}.
```

```
#trigger onInteraction{confirm_ban}
#name Confirm Ban Handler
#type interaction

Zephemeral
Zban{ZgetServerVar{pending_ban}}
Done.
```

## How it works

ZBR is powered by a high-performance runtime engine built in Rust that parses and executes bot logic, and is distributed as a convenient npm CLI. 

- **The CLI (`zbr`)**: Acts as your project manager and binary launcher.
- **The Engine**: A bundled Rust binary that handles the Discord gateway, parses your `.zbr` files, and executes logic in real-time.
- **Hot Reloading**: The engine watches your `commands/` folder. Save a `.zbr` file, and the changes are live instantly without restarting.

Commands are plain text files. Each file has a header section (lines starting with `#`) that defines the trigger, name, and type, followed by the ZBR code that runs when the command is invoked.

## Command types

| Type | Trigger format | When it runs |
|---|---|---|
| `prefix` | `!command` | When a message starts with the trigger |
| `slash` | `/command` | When a slash command is invoked |
| `interaction` | `onInteraction{id?}` | When a button, select menu, or modal is submitted |
| `event` | `onMessage`, `onMemberJoin`, etc. | When a Discord gateway event fires |

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

## What's included

- **Embeds**: Ztitle, Zdescription, Zcolor, ZaddField, ZsendEmbed, Zauthor, Zfooter
- **HTTP**: ZhttpGet, ZhttpPost, ZhttpPut, ZhttpPatch, ZhttpDelete, ZhttpResult
- **Moderation**: Zban, Zkick, Zmute, Ztimeout, Zclear, Zunban, ZvoiceKick
- **Components**: ZaddButton, ZaddSelectMenuOption, ZaddTextInput, ZaddUserSelect, ZeditButton, ZremoveComponent
- **Automod**: ZautomodRule, ZautomodRuleCreate, ZautomodRuleEdit, ZautomodRuleDelete, ZautomodRules
- **Servers**: ZserverModify, ZserverLockdown, ZeditWelcomeScreen, ZwelcomeScreen, ZboostCount, ZboostLevel
- **Channels**: ZcreateChannel, ZdeleteChannels, ZmodifyChannel, ZsyncPerms, ZchannelExists, ZchannelName, ZslowMode
- **Roles**: ZcreateRole, ZdeleteRole, ZmodifyRole, ZroleMembers, ZroleExists, ZhasRole

...and **32+ more** — see the [full function reference](https://zbr-website.vercel.app/docs).

## Example commands

The `commands/` folder contains ready-to-run examples:

| File | Type | What it does |
|---|---|---|
| `ping.zbr` | slash | Replies with pong and echoes your input |
| `hello.zbr` | prefix | Greets the user with their name, channel, and server |
| `rank.zbr` | prefix | Reads user variables and sends a rank embed |
| `eval.zbr` | prefix | Evaluates ZBR code on the fly |

## Built with

- [Rust](https://www.rust-lang.org/)
- [Serenity](https://github.com/serenity-rs/serenity), Discord API
- [Axum](https://github.com/tokio-rs/axum), HTTP runtime server
- [SQLite](https://www.sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx), variable and cooldown persistence

## License

All Rights Reserved, see [LICENSE](LICENSE).