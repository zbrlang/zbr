use crate::context::{DiscordContext, FnOutput};
use regex::Regex;
use std::net::IpAddr;

/// Zvalidate{text; type}
/// Validates if the text matches the specified type.
/// Types: "url", "ip", "email", "hexColor", "snowflake", "json"
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text = match args.get(0) {
        Some(v) => v,
        None => return FnOutput::error("validate", crate::error_messages::required(1, "text")),
    };
    
    let validation_type = match args.get(1) {
        Some(v) => v.to_lowercase(),
        None => return FnOutput::error("validate", crate::error_messages::required(2, "type")),
    };

    let is_valid = match validation_type.as_str() {
        "url" => {
            let re = Regex::new(r"^(https?://)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*/?$").unwrap();
            re.is_match(text)
        }
        "ip" => {
            text.parse::<IpAddr>().is_ok()
        }
        "email" => {
            let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
            re.is_match(text)
        }
        "hexcolor" => {
            let re = Regex::new(r"^#?([a-fA-F0-9]{3}|[a-fA-F0-9]{6})$").unwrap();
            re.is_match(text)
        }
        "snowflake" => {
            text.len() >= 17 && text.len() <= 20 && text.chars().all(|c| c.is_ascii_digit())
        }
        "json" => {
            serde_json::from_str::<serde_json::Value>(text).is_ok()
        }
        _ => return FnOutput::error("validate", crate::error_messages::expected_choice(2, "type", "url, ip, email, hexColor, snowflake, json", &validation_type)),
    };

    FnOutput::Text(is_valid.to_string())
}
