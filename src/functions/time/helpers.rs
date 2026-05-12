use chrono::Utc;
use chrono_tz::Tz;
use crate::context::DiscordContext;

/// Returns the current time in the context's active timezone.
pub fn now(ctx: &DiscordContext) -> chrono::DateTime<Tz> {
    let tz_str = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            ctx.timezone.lock().await.clone()
        })
    });

    let tz: Tz = tz_str.parse().unwrap_or(chrono_tz::Asia::Tokyo);
    Utc::now().with_timezone(&tz)
}
