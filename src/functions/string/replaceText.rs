use crate::context::{DiscordContext, FnOutput};

/// ZreplaceText{text;sample;new;(amount)}
/// Replaces occurrences of 'sample' with 'new' in 'text'.
/// Amount defaults to -1 (replace all). Use a positive integer to limit replacements.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let text   = &args[0];
    let sample = &args[1];
    let new    = &args[2];
    let amount = args.get(3)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(-1);

    if sample.is_empty() {
        return FnOutput::Text(text.clone());
    }

    let result = if amount < 0 {
        text.replace(sample.as_str(), new.as_str())
    } else {
        let mut out = text.clone();
        let mut count = 0i64;
        while count < amount {
            match out.find(sample.as_str()) {
                Some(pos) => {
                    out.replace_range(pos..pos + sample.len(), new);
                    count += 1;
                }
                None => break,
            }
        }
        out
    };

    FnOutput::Text(result)
}
