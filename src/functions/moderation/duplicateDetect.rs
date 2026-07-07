use crate::context::{ DiscordContext, FnOutput };
use once_cell::sync::Lazy;
use std::collections::{ HashMap, VecDeque };
use std::sync::{ Arc, Mutex };

/// Global cache: channel_id -> last 50 messages (user_id, content)
static MESSAGE_CACHE: Lazy<Arc<Mutex<HashMap<String, VecDeque<(String, String)>>>>> = Lazy::new(||
    Arc::new(Mutex::new(HashMap::new()))
);

const MAX_CACHE_SIZE: usize = 50;

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost
            );
        }
    }

    matrix[len1][len2]
}

fn similarity_ratio(s1: &str, s2: &str) -> f64 {
    if s1.is_empty() && s2.is_empty() {
        return 1.0;
    }
    let max_len = std::cmp::max(s1.chars().count(), s2.chars().count());
    if max_len == 0 {
        return 1.0;
    }

    let distance = levenshtein_distance(s1, s2);
    1.0 - (distance as f64) / (max_len as f64)
}

pub fn cache_message(channel_id: &str, user_id: &str, content: &str) {
    let mut cache = MESSAGE_CACHE.lock().unwrap();
    let messages = cache.entry(channel_id.to_string()).or_insert_with(VecDeque::new);

    messages.push_back((user_id.to_string(), content.to_string()));

    // Keep only last 50 messages
    while messages.len() > MAX_CACHE_SIZE {
        messages.pop_front();
    }
}

/// ZduplicateDetect{messageID?;similarityThreshold?}
/// Detects duplicate/copypasta messages using Levenshtein distance.
/// Returns "true" if duplicate detected, "false" otherwise.
pub fn run(args: Vec<String>, ctx: &DiscordContext) -> FnOutput {
    let similarity_threshold: f64 = match args.get(1) {
        Some(s) if !s.is_empty() =>
            match s.parse() {
                Ok(n) if n >= 0.0 && n <= 1.0 => n,
                _ => {
                    return FnOutput::error(
                        "duplicateDetect",
                        "similarityThreshold must be between 0.0 and 1.0"
                    );
                }
            }
        _ => 0.85,
    };

    let current_message = ctx.message.clone();
    let current_user = ctx.author_id.clone();
    let channel_id = ctx.channel_id.clone();

    if current_message.is_empty() {
        return FnOutput::Text("false".to_string());
    }

    let cache = MESSAGE_CACHE.lock().unwrap();
    let is_duplicate = if let Some(messages) = cache.get(&channel_id) {
        messages.iter().any(|(user_id, content)| {
            if user_id == &current_user {
                return false;
            }

            let ratio = similarity_ratio(&current_message, content);
            ratio >= similarity_threshold
        })
    } else {
        false
    };
    drop(cache);

    cache_message(&channel_id, &current_user, &current_message);

    FnOutput::Text((if is_duplicate { "true" } else { "false" }).to_string())
}
