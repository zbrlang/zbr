use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiExtract{apiKey;content;instruction}
/// Extracts specific information from the provided content based on the instruction.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiExtract", crate::error_messages::required(1, "apiKey"));
        }
    };

    let content = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiExtract", crate::error_messages::required(2, "content"));
        }
    };

    let instruction = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiExtract", crate::error_messages::required(3, "instruction"));
        }
    };

    let model = args
        .get(3)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    let system_prompt =
        "You are a precise information extractor. Extract ONLY the requested information from the content. Return just the extracted value(s), nothing else. If not found, return 'N/A'.";

    let prompt = format!(
        "Content: {}\n\nExtract: {}\n\nProvide only the extracted information:",
        content,
        instruction
    );

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_chat(&api_key, Some(system_prompt), &prompt, &model, Some(100), Some(0.0)) {
                Ok(text) => FnOutput::Text(text.trim().to_string()),
                Err(e) => FnOutput::error("aiExtract", e),
            }
        }
        Err(e) => FnOutput::error("aiExtract", e),
    }
}
