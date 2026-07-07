use crate::context::{ DiscordContext, FnOutput };

fn snowflake_to_timestamp(snowflake: u64) -> i64 {
    const DISCORD_EPOCH: u64 = 1420070400000;
    let timestamp_ms = (snowflake >> 22) + DISCORD_EPOCH;
    (timestamp_ms / 1000) as i64
}

/// ZnewAccountDetect{userID;minAgeDays?}
/// Detects newly created Discord accounts (common in raids/ban evasion).
/// Returns "true" if account is too new, "false" otherwise.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let user_id_str = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error(
                "newAccountDetect",
                crate::error_messages::required(1, "userID")
            );
        }
    };

    let user_id: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return FnOutput::error(
                "newAccountDetect",
                crate::error_messages::expected_snowflake(1, "userID", &user_id_str)
            );
        }
    };

    let min_age_days: i64 = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n > 0 => n,
                _ => {
                    return FnOutput::error(
                        "newAccountDetect",
                        crate::error_messages::expected_integer(2, "minAgeDays", s)
                    );
                }
            }
        _ => 7,
    };

    let account_created_timestamp = snowflake_to_timestamp(user_id);
    let now = chrono::Utc::now().timestamp();
    let account_age_seconds = now - account_created_timestamp;
    let account_age_days = account_age_seconds / 86400;

    FnOutput::Text((if account_age_days < min_age_days { "true" } else { "false" }).to_string())
}
