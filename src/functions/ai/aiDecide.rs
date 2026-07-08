use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiDecide{apiKey;content;question;model?}
/// Asks the AI a yes/no question about the provided content. Returns "true" or "false".
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiDecide", crate::error_messages::required(1, "apiKey"));
        }
    };

    let content = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiDecide", crate::error_messages::required(2, "content"));
        }
    };

    let question = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiDecide", crate::error_messages::required(3, "question"));
        }
    };

    let model = args
        .get(3)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    let system_prompt =
        "You are a strict yes/no classifier. You must respond with ONLY the word 'true' or 'false', nothing else. No punctuation, no explanation, no other words.";

    let prompt = format!(
        "Content: {}\n\nQuestion: {}\n\nAnswer with only 'true' or 'false':",
        content,
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
                Err(e) => FnOutput::error("aiDecide", e),
            }
        }
        Err(e) => FnOutput::error("aiDecide", e),
    }
}
