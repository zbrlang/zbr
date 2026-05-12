use crate::context::{DiscordContext, FnOutput};
use serenity::builder::GetMessages;
use serenity::model::id::{ChannelId, MessageId};

/// Zclear{amount;userID?;removePinned?}
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let amount_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("clear", "amount is required"),
    };

    let amount: u8 = match amount_str.parse::<u8>() {
        Ok(n) if n >= 1 => n,
        Ok(n) => {
            return FnOutput::error("clear", format!("amount must be between 1 and 100, got {}", n))
        }
        Err(_) => {
            return FnOutput::error(
                "clear",
                format!("amount must be between 1 and 100, got {}", amount_str),
            )
        }
    };

    // u8 max is 255, but Discord caps at 100
    if amount > 100 {
        return FnOutput::error("clear", format!("amount must be between 1 and 100, got {}", amount));
    }

    let filter_uid: Option<u64> = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse::<u64>() {
            Ok(id) => Some(id),
            Err(_) => return FnOutput::error("clear", format!("invalid user ID: '{}'", s)),
        },
        _ => None,
    };

    let remove_pinned: bool = match args.get(2) {
        Some(s) if !s.is_empty() => match s.as_str() {
            "true" => true,
            "false" => false,
            other => {
                return FnOutput::error(
                    "clear",
                    format!("invalid boolean for removePinned: '{}'", other),
                )
            }
        },
        _ => true,
    };

    let cid: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("clear", "invalid channel ID in context"),
    };

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("clear", "no HTTP client available"),
    };

    let result: Result<(), String> = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(cid);

            // When filtering by user we need to fetch more to have enough after filtering
            let fetch_limit = if filter_uid.is_some() { 100u8 } else { amount };
            let messages = channel
                .messages(&http, GetMessages::new().limit(fetch_limit))
                .await
                .map_err(|_| "failed to fetch messages".to_string())?;

            // Fetch pinned message IDs if we need to exclude them
            let pinned_ids: std::collections::HashSet<u64> = if !remove_pinned {
                channel
                    .pins(&http)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|m| m.id.get())
                    .collect()
            } else {
                std::collections::HashSet::new()
            };

            // Discord requires messages to be less than 14 days old for bulk delete
            let cutoff = chrono::Utc::now() - chrono::Duration::days(14);

            let to_delete: Vec<MessageId> = messages
                .into_iter()
                .filter(|m| {
                    // Must be less than 14 days old
                    if m.timestamp.unix_timestamp() < cutoff.timestamp() {
                        return false;
                    }
                    // Filter by user if requested
                    if let Some(uid) = filter_uid {
                        if m.author.id.get() != uid {
                            return false;
                        }
                    }
                    // Skip pinned messages if removePinned is false
                    if !remove_pinned && pinned_ids.contains(&m.id.get()) {
                        return false;
                    }
                    true
                })
                .take(amount as usize)
                .map(|m| m.id)
                .collect();

            if to_delete.is_empty() {
                return Ok(());
            }

            if to_delete.len() == 1 {
                // bulk_delete requires at least 2 messages; delete single message directly
                channel
                    .delete_message(&http, to_delete[0])
                    .await
                    .map_err(|_| "failed to delete messages".to_string())?;
            } else {
                channel
                    .delete_messages(&http, to_delete)
                    .await
                    .map_err(|_| "failed to delete messages".to_string())?;
            }

            Ok(())
        })
    });

    match result {
        Ok(_) => FnOutput::Empty,
        Err(e) => FnOutput::error("clear", e),
    }
}
