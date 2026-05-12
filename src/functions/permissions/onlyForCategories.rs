use crate::context::{DiscordContext, FnOutput};
use serenity::model::id::ChannelId;

/// ZonlyForCategories{categoryID1;categoryID2;...;(errorMessage)}
/// Halts unless the current channel belongs to one of the provided categories.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    if args.is_empty() {
        return FnOutput::error("onlyForCategories", "at least one category ID is required");
    }

    let (ids, error_msg) = split_ids_and_error(&args);
    if ids.is_empty() {
        return FnOutput::error("onlyForCategories", "at least one category ID is required");
    }

    let http = match &ctx.http {
        Some(h) => h.clone(),
        None => return FnOutput::error("onlyForCategories", "no HTTP client available"),
    };

    let channel_id: u64 = match ctx.channel_id.parse() {
        Ok(id) => id,
        Err(_) => return FnOutput::error("onlyForCategories", "invalid channel ID"),
    };

    let allowed_ids: Vec<String> = ids.to_vec();

    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            let channel = ChannelId::new(channel_id).to_channel(&http).await
                .map_err(|e| format!("failed to fetch channel: {}", e))?;
            let parent_id = channel.guild()
                .and_then(|c| c.parent_id)
                .map(|id| id.to_string());
            Ok::<bool, String>(
                parent_id.map(|pid| allowed_ids.iter().any(|id| id == &pid)).unwrap_or(false)
            )
        })
    });

    match result {
        Ok(true) => FnOutput::Empty,
        Ok(false) => FnOutput::user_error(error_msg),
        Err(e) => FnOutput::error("onlyForCategories", e),
    }
}

fn split_ids_and_error(args: &[String]) -> (&[String], String) {
    if let Some(last) = args.last() {
        if last.parse::<u64>().is_err() && args.len() > 1 {
            return (&args[..args.len() - 1], last.clone());
        }
    }
    (args, "This command can only be used in specific categories.".to_string())
}
