use crate::context::CooldownLabels;

/// Parse a duration string like "1h30m", "2d", "30s", "1y2w3d4h5m6s".
/// Returns total seconds, or an error string.
pub fn parse_duration(s: &str) -> Result<i64, String> {
    if s.is_empty() {
        return Err("duration cannot be empty".to_string());
    }

    let mut total: i64 = 0;
    let mut num_buf = String::new();

    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num_buf.push(ch);
        } else {
            let n: i64 = num_buf.parse().map_err(|_| format!("invalid duration: '{}'", s))?;
            num_buf.clear();
            let secs = match ch {
                'y' => n * 365 * 24 * 3600,
                'w' => n * 7   * 24 * 3600,
                'd' => n * 24  * 3600,
                'h' => n * 3600,
                'm' => n * 60,
                's' => n,
                _ => return Err(format!("unknown duration unit '{}' in '{}'", ch, s)),
            };
            total += secs;
        }
    }

    if !num_buf.is_empty() {
        return Err(format!("duration '{}' has a number with no unit", s));
    }
    if total <= 0 {
        return Err(format!("duration must be greater than zero: '{}'", s));
    }

    Ok(total)
}

/// Format remaining seconds into a human-readable string using the provided labels.
/// e.g. "2 Hours, 30 Minutes, 5 Seconds"
pub fn format_remaining(secs: i64, labels: &CooldownLabels) -> String {
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;

    let mut parts = Vec::new();
    if d > 0 { parts.push(format!("{} {}", d, labels.days)); }
    if h > 0 { parts.push(format!("{} {}", h, labels.hours)); }
    if m > 0 { parts.push(format!("{} {}", m, labels.minutes)); }
    if s > 0 || parts.is_empty() { parts.push(format!("{} {}", s, labels.seconds)); }

    parts.join(", ")
}

/// Replace %time%, %time-d%, %time-h%, %time-m%, %time-s% in an error message.
pub fn apply_time_placeholders(msg: &str, remaining_secs: i64, labels: &CooldownLabels) -> String {
    let d = remaining_secs / 86400;
    let h = (remaining_secs % 86400) / 3600;
    let m = (remaining_secs % 3600) / 60;
    let s = remaining_secs % 60;

    msg
        .replace("%time%", &format_remaining(remaining_secs, labels))
        .replace("%time-d%", &d.to_string())
        .replace("%time-h%", &h.to_string())
        .replace("%time-m%", &m.to_string())
        .replace("%time-s%", &s.to_string())
}
