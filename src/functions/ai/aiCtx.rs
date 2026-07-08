use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiCtx{apiKey;prompt;model?;maxTokens?;temperature?}
/// Sends a prompt to the AI with Discord context injected as system message.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiCtx", crate::error_messages::required(1, "apiKey"));
        }
    };

    let prompt = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiCtx", crate::error_messages::required(2, "prompt"));
        }
    };

    let model = args
        .get(2)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    let max_tokens: Option<u32> = match args.get(3) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    return FnOutput::error(
                        "aiCtx",
                        crate::error_messages::expected_integer(4, "maxTokens", s)
                    );
                }
            }
        _ => None,
    };

    let temperature: Option<f64> = match args.get(4) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) => Some(n),
                Err(_) => {
                    return FnOutput::error(
                        "aiCtx",
                        crate::error_messages::expected_number(5, "temperature", s)
                    );
                }
            }
        _ => None,
    };

    let system_prompt = build_discord_context(ctx);

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match
                gemini_chat(
                    &api_key,
                    Some(&system_prompt),
                    &prompt,
                    &model,
                    max_tokens,
                    temperature
                )
            {
                Ok(text) => FnOutput::Text(text),
                Err(e) => FnOutput::error("aiCtx", e),
            }
        }
        Err(e) => FnOutput::error("aiCtx", e),
    }
}

/// Build a system prompt from the current Discord execution context
fn build_discord_context(ctx: &DiscordContext) -> String {
    let mut parts = vec![
        "You are a helpful Discord bot assistant. Here is the current Discord context:".to_string()
    ];

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
