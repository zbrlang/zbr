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

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS spam_tracker (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            channel_id TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            has_link INTEGER NOT NULL DEFAULT 0
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create spam_tracker table");

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS raid_tracker (
            bot_id TEXT NOT NULL,
            guild_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )
    "
    )
        .execute(&pool).await
        .expect("Failed to create raid_tracker table");

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
        .unwrap_or_else(|e| {
            eprintln!("get_user_var SQL error: {}", e);
            None
        })
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
    if
        let Err(e) = sqlx
            ::query("DELETE FROM user_vars WHERE bot_id=? AND guild_id=? AND name=?")
            .bind(bot_id)
            .bind(guild_id)
            .bind(name)
            .execute(pool).await
    {
        eprintln!("reset_user_var SQL error: {}", e);
    }
}

pub async fn get_server_var(pool: &SqlitePool, bot_id: &str, guild_id: &str, name: &str) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT value FROM server_vars WHERE bot_id=? AND guild_id=? AND name=?"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or_else(|e| {
            eprintln!("get_server_var SQL error: {}", e);
            None
        })
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
    if
        let Err(e) = sqlx
            ::query("DELETE FROM server_vars WHERE bot_id=? AND name=?")
            .bind(bot_id)
            .bind(name)
            .execute(pool).await
    {
        eprintln!("reset_server_var SQL error: {}", e);
    }
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
        .unwrap_or_else(|e| {
            eprintln!("get_channel_var SQL error: {}", e);
            None
        })
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
    if
        let Err(e) = sqlx
            ::query("DELETE FROM channel_vars WHERE bot_id=? AND name=?")
            .bind(bot_id)
            .bind(name)
            .execute(pool).await
    {
        eprintln!("reset_channel_var SQL error: {}", e);
    }
}

pub async fn get_global_var(pool: &SqlitePool, bot_id: &str, name: &str) -> String {
    sqlx::query_scalar::<_, String>("SELECT value FROM global_vars WHERE bot_id=? AND name=?")
        .bind(bot_id)
        .bind(name)
        .fetch_optional(pool).await
        .unwrap_or_else(|e| {
            eprintln!("get_global_var SQL error: {}", e);
            None
        })
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
        .unwrap_or_else(|e| {
            eprintln!("list_global_vars SQL error: {}", e);
            vec![]
        })
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

pub async fn try_acquire_user_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<Option<i64>, sqlx::Error> {
    let now = chrono::Utc::now().timestamp();
    let expires_at = now + duration_secs;

    let mut tx = pool.begin().await?;

    let existing: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM user_cooldowns WHERE bot_id=? AND guild_id=? AND user_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(command)
        .fetch_optional(&mut *tx).await?;

    if let Some(e) = existing {
        if e > now {
            tx.commit().await?;
            return Ok(Some(e - now));
        }
    }

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
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(None)
}

pub async fn try_acquire_server_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<Option<i64>, sqlx::Error> {
    let now = chrono::Utc::now().timestamp();
    let expires_at = now + duration_secs;

    let mut tx = pool.begin().await?;

    let existing: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM server_cooldowns WHERE bot_id=? AND guild_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(command)
        .fetch_optional(&mut *tx).await?;

    if let Some(e) = existing {
        if e > now {
            tx.commit().await?;
            return Ok(Some(e - now));
        }
    }

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
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(None)
}

pub async fn try_acquire_global_cooldown(
    pool: &SqlitePool,
    bot_id: &str,
    user_id: &str,
    command: &str,
    duration_secs: i64
) -> Result<Option<i64>, sqlx::Error> {
    let now = chrono::Utc::now().timestamp();
    let expires_at = now + duration_secs;

    let mut tx = pool.begin().await?;

    let existing: Option<i64> = sqlx
        ::query_scalar(
            "SELECT expires_at FROM global_cooldowns WHERE bot_id=? AND user_id=? AND command=?"
        )
        .bind(bot_id)
        .bind(user_id)
        .bind(command)
        .fetch_optional(&mut *tx).await?;

    if let Some(e) = existing {
        if e > now {
            tx.commit().await?;
            return Ok(Some(e - now));
        }
    }

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
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(None)
}

/// Log a message event for spam detection
pub async fn log_spam_event(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    channel_id: &str,
    has_link: bool
) {
    let now = chrono::Utc::now().timestamp();
    let has_link_int = if has_link { 1 } else { 0 };

    let _ = sqlx
        ::query(
            "INSERT INTO spam_tracker (bot_id, guild_id, user_id, channel_id, timestamp, has_link)
         VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(channel_id)
        .bind(now)
        .bind(has_link_int)
        .execute(pool).await;
}

/// Get spam message count for a user in a time window
pub async fn get_spam_count(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    window_seconds: i64
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let cutoff = now - window_seconds;

    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM spam_tracker 
         WHERE bot_id=? AND guild_id=? AND user_id=? AND timestamp > ?"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(cutoff)
        .fetch_one(pool).await
        .unwrap_or(0)
}

/// Get link spam count for a user in a time window
pub async fn get_link_spam_count(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    user_id: &str,
    window_seconds: i64
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let cutoff = now - window_seconds;

    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM spam_tracker 
         WHERE bot_id=? AND guild_id=? AND user_id=? AND timestamp > ? AND has_link=1"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(cutoff)
        .fetch_one(pool).await
        .unwrap_or(0)
}

/// Log a member join event for raid detection
pub async fn log_raid_event(pool: &SqlitePool, bot_id: &str, guild_id: &str, user_id: &str) {
    let now = chrono::Utc::now().timestamp();

    let _ = sqlx
        ::query(
            "INSERT INTO raid_tracker (bot_id, guild_id, user_id, timestamp)
         VALUES (?, ?, ?, ?)"
        )
        .bind(bot_id)
        .bind(guild_id)
        .bind(user_id)
        .bind(now)
        .execute(pool).await;
}

/// Get raid join count in a time window
pub async fn get_raid_count(
    pool: &SqlitePool,
    bot_id: &str,
    guild_id: &str,
    window_seconds: i64
) -> i64 {
    let now = chrono::Utc::now().timestamp();
    let cutoff = now - window_seconds;

    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM raid_tracker 
         WHERE bot_id=? AND guild_id=? AND timestamp > ?"
    )
        .bind(bot_id)
        .bind(guild_id)
        .bind(cutoff)
        .fetch_one(pool).await
        .unwrap_or(0)
}

/// Clean up old spam tracker records (older than 1 hour)
pub async fn cleanup_old_spam_records(pool: &SqlitePool) {
    let now = chrono::Utc::now().timestamp();
    let cutoff = now - 3600; // 1 hour

    let _ = sqlx
        ::query("DELETE FROM spam_tracker WHERE timestamp < ?")
        .bind(cutoff)
        .execute(pool).await;
}

/// Clean up old raid tracker records (older than 1 hour)
pub async fn cleanup_old_raid_records(pool: &SqlitePool) {
    let now = chrono::Utc::now().timestamp();
    let cutoff = now - 3600; // 1 hour

    let _ = sqlx
        ::query("DELETE FROM raid_tracker WHERE timestamp < ?")
        .bind(cutoff)
        .execute(pool).await;
}
