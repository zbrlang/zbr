use crate::context::{DiscordContext, FnOutput};

/// ZsuppressErrors{errorText?;embedIndex?}
/// Activates error suppression for this execution.
///
/// No args          → suppress silently (nothing shown on error)
/// errorText only   → show this text on error instead of the real error
/// ;embedIndex      → send this embed slot (1-based) on error
/// errorText;index  → show text AND send embed on error
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let error_text = match args.get(0) {
        Some(s) if !s.is_empty() => Some(s.clone()),
        _ => Some(String::new()), // Some("") = suppress silently
    };

    let embed_index: Option<usize> = match args.get(1) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => Some(n - 1), // convert to 0-based
            Ok(_) => return FnOutput::error("suppressErrors", "embed index must be 1 or greater"),
            Err(_) => return FnOutput::error("suppressErrors", format!("invalid embed index: '{}'", s)),
        },
        _ => None,
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            *ctx.suppress_error_text.lock().await = error_text;
            *ctx.suppress_error_embed.lock().await = embed_index;
        })
    });

    FnOutput::Empty
}
