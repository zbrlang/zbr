use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_chat, AiProvider };

/// ZaiClassify{apiKey;content;categories}
/// Classifies the content into one of the provided categories (comma-separated).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiClassify", crate::error_messages::required(1, "apiKey"));
        }
    };

    let content = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiClassify", crate::error_messages::required(2, "content"));
        }
    };

    let categories = match args.get(2) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("aiClassify", crate::error_messages::required(3, "categories"));
        }
    };

    let model = args
        .get(3)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    // Parse categories
    let category_list: Vec<&str> = categories
        .split(',')
        .map(|s| s.trim())
        .collect();

    if category_list.is_empty() {
        return FnOutput::error("aiClassify", "categories cannot be empty");
    }

    let system_prompt = format!(
        "You are a strict classifier. You must respond with ONLY one of these exact categories: {}. No punctuation, no explanation, no other words. Just pick the single best matching category.",
        category_list.join(", ")
    );

    let prompt = format!(
        "Content: {}\n\nAvailable categories: {}\n\nClassify into exactly one category:",
        content,
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
                Err(e) => FnOutput::error("aiClassify", e),
            }
        }
        Err(e) => FnOutput::error("aiClassify", e),
    }
}
