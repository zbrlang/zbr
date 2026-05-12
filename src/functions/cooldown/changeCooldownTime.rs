use crate::context::{DiscordContext, FnOutput};

/// ZchangeCooldownTime{days;hours;minutes;seconds}
/// Overrides the label text used in cooldown time formatting.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let days    = args.get(0).cloned().unwrap_or_else(|| "Days".to_string());
    let hours   = args.get(1).cloned().unwrap_or_else(|| "Hours".to_string());
    let minutes = args.get(2).cloned().unwrap_or_else(|| "Minutes".to_string());
    let seconds = args.get(3).cloned().unwrap_or_else(|| "Seconds".to_string());

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let mut labels = ctx.cooldown_labels.lock().await;
            labels.days    = days;
            labels.hours   = hours;
            labels.minutes = minutes;
            labels.seconds = seconds;
        })
    });

    FnOutput::Empty
}
