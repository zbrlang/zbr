use crate::context::{ DiscordContext, FnOutput };
use super::helpers::{ detect_provider, gemini_vision, AiProvider };

/// ZimageAnalyze{apiKey;imageURL;prompt?}
/// Analyzes an image and returns a text description.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let api_key = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("imageAnalyze", crate::error_messages::required(1, "apiKey"));
        }
    };

    let image_url = match args.get(1) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            return FnOutput::error("imageAnalyze", crate::error_messages::required(2, "imageURL"));
        }
    };

    let prompt = args
        .get(2)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_else(|| "Describe this image.".to_string());

    let model = args
        .get(3)
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default();

    match detect_provider(&api_key) {
        Ok(AiProvider::Gemini) => {
            match gemini_vision(&api_key, &image_url, &prompt, &model) {
                Ok(text) => FnOutput::Text(text),
                Err(e) => FnOutput::error("imageAnalyze", e),
            }
        }
        Err(e) => FnOutput::error("imageAnalyze", e),
    }
}
