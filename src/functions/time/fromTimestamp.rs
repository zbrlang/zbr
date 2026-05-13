use crate::context::{DiscordContext, FnOutput};
use crate::functions::time::helpers::now;
use chrono::{TimeZone, Utc};

/// ZfromTimestamp{unix;format?}
/// Formats a Unix timestamp into a date/time string using the context timezone.
/// format: full (default), date, time, relative
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let unix: i64 = match args.get(0).and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return FnOutput::error("fromTimestamp", "unix timestamp is required"),
    };
    let format = args.get(1).map(|s| s.as_str()).unwrap_or("full");

    let tz_str = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async { ctx.timezone.lock().await.clone() })
    });
    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::Asia::Tokyo);
    let dt = Utc
        .timestamp_opt(unix, 0)
        .single()
        .map(|dt| dt.with_timezone(&tz));

    match dt {
        Some(dt) => {
            let s = match format {
                "date" => dt.format("%Y-%m-%d").to_string(),
                "time" => dt.format("%H:%M:%S").to_string(),
                "relative" => {
                    let now_ts = now(ctx).timestamp();
                    let diff = (unix - now_ts).abs();
                    if diff < 60 {
                        format!("{diff}s ago")
                    } else if diff < 3600 {
                        format!("{}m ago", diff / 60)
                    } else if diff < 86400 {
                        format!("{}h ago", diff / 3600)
                    } else {
                        format!("{}d ago", diff / 86400)
                    }
                }
                _ => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            };
            FnOutput::Text(s)
        }
        None => FnOutput::error("fromTimestamp", "invalid unix timestamp"),
    }
}
