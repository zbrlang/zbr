use crate::context::{DiscordContext, FnOutput};

/// Zurl{mode;text}
/// mode: "encode" or "decode"
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let mode = match args.get(0) {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return FnOutput::error("url", crate::error_messages::required(1, "mode")),
    };
    let text = args.get(1).filter(|s| !s.is_empty()).cloned().unwrap_or_default();

    match mode.as_str() {
        "encode" => FnOutput::Text(
            text.chars()
                .flat_map(|c| {
                    let mut buf = [0u8; 4];
                    let s = c.encode_utf8(&mut buf);
                    s.bytes()
                        .map(|b| {
                            if b.is_ascii_alphanumeric()
                                || b == b'-'
                                || b == b'_'
                                || b == b'.'
                                || b == b'~'
                            {
                                (b as char).to_string()
                            } else {
                                format!("%{:02X}", b)
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<String>(),
        ),
        "decode" => {
            let bytes = text.as_bytes();
            let mut out = String::new();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'%' && i + 2 < bytes.len() {
                    if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                        if let Ok(byte) = u8::from_str_radix(hex, 16) {
                            out.push(byte as char);
                            i += 3;
                            continue;
                        }
                    }
                } else if bytes[i] == b'+' {
                    out.push(' ');
                    i += 1;
                    continue;
                }
                out.push(bytes[i] as char);
                i += 1;
            }
            FnOutput::Text(out)
        }
        other => FnOutput::error("url", crate::error_messages::expected_choice(1, "mode", "encode, decode", other)),
    }
}
