# Privacy Policy

Last Updated: May 26, 2026

Your privacy is important to us. This policy explains what data ZBR collects, how it is used, and your rights regarding that data.

## 1. Data We Collect
ZBR collects and stores data necessary to provide its scripting and automation services:
- **Discord Identifiers**: User IDs, Guild IDs, Channel IDs, and Role IDs.
- **Message Content**: We process message content to detect script triggers. Content is only stored if explicitly saved by a script using our variable system.
- **Custom Variables**: Any data an administrator or script developer chooses to store using ZBR functions (`ZsetVar`, `ZsetUserVar`, `ZsetServerVar`, `ZsetChannelVar`).
- **Cooldown Information**: Timestamps of command usage to enforce rate limits.
- **Interaction Data**: Data submitted through Discord components like Modals and Select Menus.

## 2. How We Use Data
- **Service Execution**: To run your custom scripts and respond to commands.
- **Persistence**: To remember user settings, experience points, or any other custom data defined in your scripts.
- **Rate Limiting**: To prevent abuse and manage bot performance.
- **Moderation**: To execute moderation actions as directed by server administrators.

## 3. Data Storage and Security
- **SQLite Database**: All persistent data is stored in a local SQLite database.
- **Encryption**: We recommend securing the environment where the bot is hosted, as the database itself is stored in plain text by default.
- **Data Retention**: Data in variables is stored until explicitly deleted by a script (e.g., `ZresetUserVar`) or until the database is cleared.
- **SSRF Protection**: HTTP functions include built-in URL validation that blocks requests to private, reserved, or known-dangerous IP addresses to prevent server-side request forgery attacks.
- **Header Filtering**: Dangerous HTTP headers (cookie, host, connection, etc.) are blocked from being set by scripts to prevent request smuggling and header injection.

## 4. Third-Party Sharing
- **Discord**: Data is shared with Discord as part of standard bot operations (sending messages, managing roles, modifying automod rules, managing polls, soundboard sounds, and stickers).
- **External APIs**: If a script uses ZBR's HTTP functions (`ZhttpPost`, `ZhttpGet`, etc.), data may be sent to external URLs specified in the script. All URLs are validated against an SSRF blocklist before requests are made. Users should still be aware of where their scripts are sending data.
- **File Uploads**: Functions that accept file URLs (`ZstickerCreate`, `ZsoundboardCreate`) will download content from the specified URL and relay it to Discord. The content itself is not stored by the ZBR engine.
- **No Sale of Data**: We do not sell, trade, or otherwise transfer your personal information to outside parties for marketing purposes.

## 5. User Rights and Control
- **Access and Deletion**: Users can request the deletion of their data from a server administrator. Administrators can use ZBR functions to clear stored user data.
- **Opt-Out**: Users can stop interacting with the bot or leave the server to prevent further data collection.

## 6. Compliance with Discord
ZBR complies with the Discord Developer Policy and Terms of Service. We do not collect more data than is necessary for the bot's functionality.

## 7. Contact Information
For questions regarding this policy or data deletion requests, please contact the bot host/administrator of your specific server.
