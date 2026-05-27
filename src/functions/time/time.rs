use crate::context::{DiscordContext, FnOutput};

/// Ztime{America/New_York} — sets the timezone for all subsequent date/time functions.
/// Ztime{} — returns the current timezone name.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let tz_str = args.get(0).cloned().unwrap_or_default();

    if tz_str.is_empty() {
        let current = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                ctx.timezone.lock().await.clone()
            })
        });
        return FnOutput::Text(current);
    }

    // Validate the timezone string before storing it
    if tz_str.parse::<chrono_tz::Tz>().is_err() {
        return FnOutput::error("time", crate::error_messages::not_found("timezone", &tz_str));
    }

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.timezone.lock().await = tz_str;
        })
    });

    FnOutput::Empty
}
