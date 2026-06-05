use sqlx::{ sqlite::{ SqliteConnectOptions, SqlitePoolOptions }, SqlitePool };
use std::str::FromStr;

pub async fn connect() -> SqlitePool {
    let url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL in .env");

    let options = SqliteConnectOptions::from_str(&url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options).await
        .expect("Failed to connect to database");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS user_vars (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT NOT NULL DEFAULT '',
            PRIMARY KEY (bot_id, guild_id, user_id, name)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create user_vars table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS server_vars (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT NOT NULL DEFAULT '',
            PRIMARY KEY (bot_id, guild_id, name)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create server_vars table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS channel_vars (
            bot_id TEXT NOT NULL,
            channel_id TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT NOT NULL DEFAULT '',
            PRIMARY KEY (bot_id, channel_id, name)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create channel_vars table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS global_vars (
            bot_id TEXT NOT NULL,
            name TEXT NOT NULL,
            value TEXT NOT NULL DEFAULT '',
            PRIMARY KEY (bot_id, name)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create global_vars table");

    // ── Cooldown tables ───────────────────────────────────────────────────────
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS user_cooldowns (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            command TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            PRIMARY KEY (bot_id, guild_id, user_id, command)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create user_cooldowns table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS server_cooldowns (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            command TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            PRIMARY KEY (bot_id, guild_id, command)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create server_cooldowns table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS global_cooldowns (
            bot_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            command TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            PRIMARY KEY (bot_id, user_id, command)
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create global_cooldowns table");

    println!("Database connected");
    pool
}

pub async fn get_user_var(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    name: &str
) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT value FROM user_vars WHERE bot_id=? AND guild_id=? AND user_id=? AND name=?"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or(None)
        .unwrap_or_default()
}

pub async fn set_user_var(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    name: &str,
    value: &str
) -> Result<(), sqlx::Error> {
    sqlx
        ::query(
            "INSERT INTO user_vars (bot_id, guild_id, user_id, name, value)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(bot_id, guild_id, user_id, name) DO UPDATE SET value=excluded.value"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(name)
        .bind(value)
        .execute(pool).await?;
    Ok(())
}

/// Delete all user_vars rows for a given bot+guild+name (resets for every user).
pub async fn reset_user_var(pool: &SqlitePool, bot_id: &str, guild_id: &str, name: &str) {
    sqlx::query("DELETE FROM user_vars WHERE bot_id=? AND guild_id=? AND name=?")
        .bind(bot_id)
        .bind(guild_id)
        .bind(name)
        .execute(pool).await
        .ok();
}

pub async fn get_server_var(pool: &SqlitePool, bot_id: &str, guild_id: &str, name: &str) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT value FROM server_vars WHERE bot_id=? AND guild_id=? AND name=?"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or(None)
        .unwrap_or_default()
}

pub async fn set_server_var(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    name: &str,
    value: &str
) -> Result<(), sqlx::Error> {
    sqlx
        ::query(
            "INSERT INTO server_vars (bot_id, guild_id, name, value)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(bot_id, guild_id, name) DO UPDATE SET value=excluded.value"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(name)
        .bind(value)
        .execute(pool).await?;
    Ok(())
}

/// Delete all server_vars rows for a given bot+name (resets across every server).
pub async fn reset_server_var(pool: &SqlitePool, bot_id: &str, name: &str) {
    sqlx::query("DELETE FROM server_vars WHERE bot_id=? AND name=?")
        .bind(bot_id)
        .bind(name)
        .execute(pool).await
        .ok();
}

pub async fn get_channel_var(
    pool: &SqlitePool,
    bot_id: &str,
    channel_id: &str,
    name: &str
) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT value FROM channel_vars WHERE bot_id=? AND channel_id=? AND name=?"
    )
        .bind(bot_id)
        .bind(channel_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or(None)
        .unwrap_or_default()
}

pub async fn set_channel_var(
    pool: &SqlitePool,
    bot_id: &str,
    channel_id: &str,
    name: &str,
    value: &str
) -> Result<(), sqlx::Error> {
    sqlx
        ::query(
            "INSERT INTO channel_vars (bot_id, channel_id, name, value)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(bot_id, channel_id, name) DO UPDATE SET value=excluded.value"
        )
        .bind(bot_id)
        .bind(channel_id)
        .bind(name)
        .bind(value)
        .execute(pool).await?;
    Ok(())
}

/// Delete all channel_vars rows for a given bot+name (resets across every channel).
pub async fn reset_channel_var(pool: &SqlitePool, bot_id: &str, name: &str) {
    sqlx::query("DELETE FROM channel_vars WHERE bot_id=? AND name=?")
        .bind(bot_id)
        .bind(name)
        .execute(pool).await
        .ok();
}

pub async fn get_global_var(pool: &SqlitePool, bot_id: &str, name: &str) -> String {
    sqlx::query_scalar::<_, String>("SELECT value FROM global_vars WHERE bot_id=? AND name=?")
        .bind(bot_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or(None)
        .unwrap_or_default()
}

pub async fn set_global_var(
    pool: &SqlitePool,
    bot_id: &str,
    name: &str,
    value: &str
) -> Result<(), sqlx::Error> {
    sqlx
        ::query(
            "INSERT INTO global_vars (bot_id, name, value)
         VALUES (?, ?, ?)
         ON CONFLICT(bot_id, name) DO UPDATE SET value=excluded.value"
        )
        .bind(bot_id)
        .bind(name)
        .bind(value)
        .execute(pool).await?;
    Ok(())
}

/// Returns all global var names for a bot.
pub async fn list_global_vars(pool: &SqlitePool, bot_id: &str) -> Vec<String> {
    sqlx::query_scalar::<_, String>("SELECT name FROM global_vars WHERE bot_id=? ORDER BY name")
        .bind(bot_id)
        .fetch_all(pool).await
        .unwrap_or_default()
}

/// Returns true if a global var exists for this bot.
pub async fn global_var_exists(pool: &SqlitePool, bot_id: &str, name: &str) -> bool {
    sqlx
        ::query_scalar::<_, i64>("SELECT COUNT(*) FROM global_vars WHERE bot_id=? AND name=?")
        .bind(bot_id)
        .bind(name)
        .fetch_one(pool).await
        .unwrap_or(0) > 0
}

// ── Cooldown DB functions ─────────────────────────────────────────────────────

/// Returns remaining seconds on a user cooldown, or 0 if not active.
pub async fn get_user_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    command: &str
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let expires: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM user_cooldowns WHERE bot_id=? AND guild_id=? AND user_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(command)
        .fetch_optional(pool).await
        .unwrap_or(None);
    match expires {
        Some(e) if e > now => e - now,
        _ => 0,
    }
}

pub async fn set_user_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<(), sqlx::Error> {
    let expires_at = chrono::Utc::now().timestamp() + duration_secs;
    sqlx
        ::query(
            "INSERT INTO user_cooldowns (bot_id, guild_id, user_id, command, expires_at)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(bot_id, guild_id, user_id, command) DO UPDATE SET expires_at=excluded.expires_at"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(command)
        .bind(expires_at)
        .execute(pool).await?;
    Ok(())
}

/// Returns remaining seconds on a server cooldown, or 0 if not active.
pub async fn get_server_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    command: &str
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let expires: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM server_cooldowns WHERE bot_id=? AND guild_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(command)
        .fetch_optional(pool).await
        .unwrap_or(None);
    match expires {
        Some(e) if e > now => e - now,
        _ => 0,
    }
}

pub async fn set_server_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<(), sqlx::Error> {
    let expires_at = chrono::Utc::now().timestamp() + duration_secs;
    sqlx
        ::query(
            "INSERT INTO server_cooldowns (bot_id, guild_id, command, expires_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(bot_id, guild_id, command) DO UPDATE SET expires_at=excluded.expires_at"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(command)
        .bind(expires_at)
        .execute(pool).await?;
    Ok(())
}

/// Returns remaining seconds on a global cooldown, or 0 if not active.
pub async fn get_global_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    user_id: &str,
    command: &str
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let expires: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM global_cooldowns WHERE bot_id=? AND user_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(user_id)
        .bind(command)
        .fetch_optional(pool).await
        .unwrap_or(None);
    match expires {
        Some(e) if e > now => e - now,
        _ => 0,
    }
}

pub async fn set_global_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    user_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<(), sqlx::Error> {
    let expires_at = chrono::Utc::now().timestamp() + duration_secs;
    sqlx
        ::query(
            "INSERT INTO global_cooldowns (bot_id, user_id, command, expires_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(bot_id, user_id, command) DO UPDATE SET expires_at=excluded.expires_at"
        )
        .bind(bot_id)
        .bind(user_id)
        .bind(command)
        .bind(expires_at)
        .execute(pool).await?;
    Ok(())
}
