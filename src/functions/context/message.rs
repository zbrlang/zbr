use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    match args.get(0) {
        Some(idx_str) => {
            match idx_str.parse::<usize>() {
                Ok(idx) if idx > 0 => {
                    // For slash commands: return the Nth option value (1-indexed)
                    if !ctx.options_list.is_empty() {
                        return FnOutput::Text(
                            ctx.options_list.get(idx - 1).cloned().unwrap_or_default()
                        );
                    }
                    // For prefix commands: return the Nth word after the trigger
                    let content = strip_trigger(&ctx.message, &ctx.trigger);
                    let words: Vec<&str> = content.split_whitespace().collect();
                    FnOutput::Text(words.get(idx - 1).copied().unwrap_or("").to_string())
                }
                _ => FnOutput::Text(String::new()),
            }
        }
        None => {
            // Zmessage{} — full content after trigger (prefix) or all options joined (slash)
            if !ctx.options_list.is_empty() {
                return FnOutput::Text(ctx.options_list.join(" "));
            }
            let content = strip_trigger(&ctx.message, &ctx.trigger);
            FnOutput::Text(content.to_string())
        }
    }
}

fn strip_trigger<'a>(message: &'a str, trigger: &Option<String>) -> &'a str {
    let message = message.trim();
    if let Some(t) = trigger {
        if message.starts_with(t.as_str()) {
            return message[t.len()..].trim_start();
        }
    }
    message
}
