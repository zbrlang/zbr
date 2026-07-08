use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiExtractCtx{apiKey;instruction}
/// Extracts specific information from the current Discord context based on the instruction.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiExtractCtx", crate::error_messages::required(1, "apiKey"));
        }
    };

    let instruction = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error(
                "aiExtractCtx",
                crate::error_messages::required(2, "instruction")
            );
        }
    };

    let model = args
        .get(2)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    let system_prompt =
        "You are a precise information extractor. Extract ONLY the requested information from the Discord context. Return just the extracted value(s), nothing else. If not found, return 'N/A'.";

    // Build Discord context
    let context_str = build_discord_context(ctx);

    let prompt = format!(
        "{}\n\nExtract: {}\n\nProvide only the extracted information:",
        context_str,
        instruction
    );

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_chat(&api_key, Some(system_prompt), &prompt, &model, Some(100), Some(0.0)) {
                Ok(text) => FnOutput::Text(text.trim().to_string()),
                Err(e) => FnOutput::error("aiExtractCtx", e),
            }
        }
        Err(e) => FnOutput::error("aiExtractCtx", e),
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
