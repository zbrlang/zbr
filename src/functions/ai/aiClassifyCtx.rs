use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiClassifyCtx{apiKey;categories}
/// Classifies the current Discord context into one of the provided categories (comma-separated).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiClassifyCtx", crate::error_messages::required(1, "apiKey"));
        }
    };

    let categories = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error(
                "aiClassifyCtx",
                crate::error_messages::required(2, "categories")
            );
        }
    };

    let model = args
        .get(2)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    // Parse categories
    let category_list: Vec<&str> = categories
        .split(',')
        .map(|s| s.trim())
        .collect();

    if category_list.is_empty() {
        return FnOutput::error("aiClassifyCtx", "categories cannot be empty");
    }

    let system_prompt = format!(
        "You are a strict classifier. You must respond with ONLY one of these exact categories: {}. No punctuation, no explanation, no other words. Just pick the single best matching category.",
        category_list.join(", ")
    );

    // Build Discord context
    let context_str = build_discord_context(ctx);

    let prompt = format!(
        "{}\n\nAvailable categories: {}\n\nClassify into exactly one category:",
        context_str,
        category_list.join(", ")
    );

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_chat(&api_key, Some(&system_prompt), &prompt, &model, Some(20), Some(0.0)) {
                Ok(text) => {
                    let result = text.trim().to_lowercase();

                    // Validate that the result matches one of the categories
                    for category in &category_list {
                        if result.contains(&category.to_lowercase()) {
                            return FnOutput::Text(category.to_string());
                        }
                    }

                    // If no exact match, return the raw result
                    FnOutput::Text(text.trim().to_string())
                }
                Err(e) => FnOutput::error("aiClassifyCtx", e),
            }
        }
        Err(e) => FnOutput::error("aiClassifyCtx", e),
    }
}

/// Build a Discord context string from the current execution context
fn build_discord_context(ctx: &DiscordContext) -> String {
    let mut parts = vec!["Discord context:".to_string()];

    // Author information
    if !ctx.author_id.is_empty() {
        parts.push(format!("Author User ID: {}", ctx.author_id));
    }
    if !ctx.username.is_empty() {
        parts.push(format!("Author Username: {}", ctx.username));
    }

    // Server/Channel information
    if !ctx.guild_id.is_empty() {
        parts.push(format!("Server ID: {}", ctx.guild_id));
    }
    if !ctx.channel_id.is_empty() {
        parts.push(format!("Channel ID: {}", ctx.channel_id));
    }

    // Message information
    if !ctx.message.is_empty() {
        parts.push(format!("Message Content: {}", ctx.message));
    }
    if let Some(msg_id) = &ctx.trigger_message_id {
        parts.push(format!("Message ID: {}", msg_id));
    }

    // Command context
    if !ctx.command_name.is_empty() {
        parts.push(format!("Command: {}", ctx.command_name));
    }
    if let Some(trigger) = &ctx.trigger {
        parts.push(format!("Trigger Prefix: {}", trigger));
    }

    parts.join("\n")
}
