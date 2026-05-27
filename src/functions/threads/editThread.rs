use crate::context::{DiscordContext, FnOutput};
use crate::functions::embeds::helpers::{validate_bool, validate_snowflake};
use serenity::builder::EditThread;
use serenity::model::channel::AutoArchiveDuration;
use serenity::model::id::ChannelId;

/// ZeditThread{threadID;(name);(archived);(archiveDuration);(locked);(slowmode)}
/// Use "!unchanged" for any field you want to leave as-is.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let thread_id_str = args.get(0).cloned().unwrap_or_default();
    let thread_id = match validate_snowflake(&thread_id_str, "editThread", "thread ID") {
        Ok(id) => id, Err(e) => return e,
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("editThread", crate::error_messages::action_failed("get HTTP client")),
    };

    // Parse each optional field — "!unchanged" or missing = skip
    let name = args.get(1).filter(|s| !s.is_empty() && s.as_str() != "!unchanged").cloned();

    let archived: Option<bool> = match args.get(2).map(|s| s.as_str()) {
        Some(s) if s != "!unchanged" && !s.is_empty() => match validate_bool(s, "editThread") {
            Ok(b) => Some(b), Err(e) => return e,
        },
        _ => None,
    };

    let archive_duration: Option<AutoArchiveDuration> = match args.get(3).map(|s| s.as_str()) {
        Some("60")    => Some(AutoArchiveDuration::OneHour),
        Some("1440")  => Some(AutoArchiveDuration::OneDay),
        Some("4320")  => Some(AutoArchiveDuration::ThreeDays),
        Some("10080") => Some(AutoArchiveDuration::OneWeek),
        Some(s) if s != "!unchanged" && !s.is_empty() =>
            return FnOutput::error("editThread", crate::error_messages::expected_choice(4, "archive duration", "60, 1440, 4320, 10080", s)),
        _ => None,
    };

    let locked: Option<bool> = match args.get(4).map(|s| s.as_str()) {
        Some(s) if s != "!unchanged" && !s.is_empty() => match validate_bool(s, "editThread") {
            Ok(b) => Some(b), Err(e) => return e,
        },
        _ => None,
    };

    let slowmode: Option<u16> = match args.get(5).map(|s| s.as_str()) {
        Some(s) if s != "!unchanged" && !s.is_empty() => match s.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_) => return FnOutput::error("editThread", crate::error_messages::expected_integer(6, "slowmode", s)),
        },
        _ => None,
    };

    // If nothing to change, return early
    if name.is_none() && archived.is_none() && archive_duration.is_none()
        && locked.is_none() && slowmode.is_none() {
        return FnOutput::Empty;
    }

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let mut builder = EditThread::new();
            if let Some(n) = name { builder = builder.name(n); }
            if let Some(a) = archived { builder = builder.archived(a); }
            if let Some(d) = archive_duration { builder = builder.auto_archive_duration(d); }
            if let Some(l) = locked { builder = builder.locked(l); }
            if let Some(s) = slowmode { builder = builder.rate_limit_per_user(s); }

            ChannelId::new(thread_id).edit_thread(&http, builder).await
                .map_err(|e| crate::error_messages::action_failed_reason("edit thread", &e.to_string()))
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("editThread", e),
    }
}
