use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// Zai{apiKey;prompt;model?;maxTokens?;temperature?}
/// Sends a prompt to the AI and returns the response text.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("ai", crate::error_messages::required(1, "apiKey"));
        }
    };

    let prompt = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("ai", crate::error_messages::required(2, "prompt"));
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
                        "ai",
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
                        "ai",
                        crate::error_messages::expected_number(5, "temperature", s)
                    );
                }
            }
        _ => None,
    };

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_chat(&api_key, None, &prompt, &model, max_tokens, temperature) {
                Ok(text) => FnOutput::Text(text),
                Err(e) => FnOutput::error("ai", e),
            }
        }
        Err(e) => FnOutput::error("ai", e),
    }
}
