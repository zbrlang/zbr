use crate::context::{DiscordContext, FnOutput};

/// Zmentioned{index;fallbackToAuthor?}
/// Returns the user ID of the Nth mentioned user in the triggering message (1-based).
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let index: usize = match args.get(0) {
        Some(s) if !s.is_empty() => match s.parse::<usize>() {
            Ok(n) if n >= 1 => n,
            _ => return FnOutput::error("mentioned", format!("invalid index: '{}' (must be 1 or greater)", s)),
        },
        _ => return FnOutput::error("mentioned", "index is required"),
    };

    let fallback = match args.get(1) {
        Some(s) if s == "true" => true,
        _ => false,
    };

    // Extract all <@userID> and <@!userID> mentions from the message
    let msg = &ctx.message;
    let mut mentions: Vec<String> = Vec::new();
    let mut chars = msg.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '<' {
            if let Some(&(_, '@')) = chars.peek() {
                chars.next(); // consume '@'
                // optionally consume '!'
                let has_bang = chars.peek().map(|&(_, c)| c == '!').unwrap_or(false);
                if has_bang {
                    chars.next();
                }
                // collect digits
                let start = chars.peek().map(|&(i, _)| i).unwrap_or(i);
                let mut end = start;
                let mut digits = String::new();
                while let Some(&(j, d)) = chars.peek() {
                    if d.is_ascii_digit() {
                        digits.push(d);
                        end = j;
                        chars.next();
                    } else {
                        break;
                    }
                }
                // expect closing '>'
                if chars.peek().map(|&(_, c)| c == '>').unwrap_or(false) {
                    chars.next();
                    if !digits.is_empty() {
                        mentions.push(digits);
                    }
                }
                let _ = (start, end);
            }
        }
    }

    if let Some(uid) = mentions.get(index - 1) {
        FnOutput::Text(uid.clone())
    } else if fallback {
        FnOutput::Text(ctx.author_id.clone())
    } else {
        FnOutput::Text(String::new())
    }
}
