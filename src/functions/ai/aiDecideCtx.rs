use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiDecideCtx{apiKey;question;model?}
/// Asks the AI a yes/no question about the current Discord context. Returns "true" or "false".
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiDecideCtx", crate::error_messages::required(1, "apiKey"));
        }
    };

    let question = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiDecideCtx", crate::error_messages::required(2, "question"));
        }
    };

    let model = args
        .get(2)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    let system_prompt =
        "You are a strict yes/no classifier. You must respond with ONLY the word 'true' or 'false', nothing else. No punctuation, no explanation, no other words.";

    let mut context_parts = vec![];

    if !ctx.author_id.is_empty() {
        context_parts.push(format!("Author User ID: {}", ctx.author_id));
    }
    if !ctx.username.is_empty() {
        context_parts.push(format!("Author Username: {}", ctx.username));
    }

    if !ctx.guild_id.is_empty() {
        context_parts.push(format!("Server ID: {}", ctx.guild_id));
    }
    if !ctx.channel_id.is_empty() {
        context_parts.push(format!("Channel ID: {}", ctx.channel_id));
    }

    if !ctx.message.is_empty() {
        context_parts.push(format!("Message Content: {}", ctx.message));
    }
    if let Some(msg_id) = &ctx.trigger_message_id {
        context_parts.push(format!("Message ID: {}", msg_id));
    }

    if !ctx.command_name.is_empty() {
        context_parts.push(format!("Command: {}", ctx.command_name));
    }

    let context_str = context_parts.join("\n");

    let prompt = format!(
        "Discord context:\n{}\n\nQuestion: {}\n\nAnswer with only 'true' or 'false':",
        context_str,
        question
    );

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_chat(&api_key, Some(system_prompt), &prompt, &model, Some(5), Some(0.0)) {
                Ok(text) => {
                    let normalized = text.trim().to_lowercase();
                    if normalized.starts_with("true") {
                        FnOutput::Text("true".to_string())
                    } else {
                        FnOutput::Text("false".to_string())
                    }
                }
                Err(e) => FnOutput::error("aiDecideCtx", e),
            }
        }
        Err(e) => FnOutput::error("aiDecideCtx", e),
    }
}
