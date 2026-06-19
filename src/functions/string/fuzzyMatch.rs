use crate::context::{DiscordContext, FnOutput};

/// ZfuzzyMatch{text1; text2}
/// Calculates Levenshtein distance and returns similarity score 0-100.
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let s1 = &args[0];
    let s2 = &args[1];

    if s1.is_empty() && s2.is_empty() {
        return FnOutput::Text("100".to_string());
    }

    let dist = levenshtein(s1, s2);
    let max_len = s1.chars().count().max(s2.chars().count());
    
    if max_len == 0 {
        return FnOutput::Text("100".to_string());
    }

    let similarity = (1.0 - (dist as f64 / max_len as f64)) * 100.0;

    FnOutput::Text(format!("{:.0}", similarity))
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();
    if n == 0 { return m; }
    if m == 0 { return n; }

    let mut dp = vec![0; m + 1];
    for j in 0..=m { dp[j] = j; }

    for i in 1..=n {
        let mut prev = i;
        let mut diag = i - 1;
        for j in 1..=m {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            let current = (prev + 1).min(dp[j] + 1).min(diag + cost);
            diag = dp[j];
            dp[j] = current;
            prev = current;
        }
    }
    dp[m]
}
