use crate::ast::Node;

fn strip_inline_comments(line: &str) -> &str {
    let mut depth = 0i32;
    let mut chars = line.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        // Match the parser's existing escape handling: \{, \;, and \\ are treated
        // as literals, and the escaped '{' must not affect brace depth.
        if ch == '\\' {
            if let Some(&(_, next)) = chars.peek() {
                if next == '{' || next == ';' || next == '\\' {
                    chars.next(); // consume the escaped character
                    continue;
                }
            }
        }

        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            '/' if depth == 0 => {
                if let Some(&(_, next)) = chars.peek() {
                    if next == '/' {
                        return line[..i].trim_end();
                    }
                }
            }
            _ => {}
        }
    }

    line
}

pub fn parse_line(line: &str) -> Option<Node> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    if line.starts_with("//") {
        return None;
    }

    let line = strip_inline_comments(line);
    if line.is_empty() {
        return None;
    }

    let segments = parse_template(line);
    if segments.is_empty() {
        return None;
    }
    if segments.len() == 1 {
        return Some(segments.into_iter().next().unwrap());
    }

    Some(Node::Concat(segments))
}

fn parse_template(line: &str) -> Vec<Node> {
    let mut segments = Vec::new();
    let mut chars = line.char_indices().peekable();
    let mut current = String::new();

    while let Some((i, ch)) = chars.next() {
        // Escape sequences: \{ → {, \; → ;, \\ → \
        if ch == '\\' {
            if let Some((_, next)) = chars.peek().copied() {
                if next == '{' || next == ';' || next == '\\' {
                    chars.next();
                    current.push(next);
                    continue;
                }
            }
            // Not a recognised escape — treat backslash as literal
            current.push(ch);
            continue;
        }

        if ch == 'Z'
            && chars
                .peek()
                .map(|(_, c)| c.is_alphabetic())
                .unwrap_or(false)
        {
            // check if this looks like a Z function call
            let rest = &line[i..];
            if let Some(end) = find_call_end(rest) {
                let call_str = &rest[..end];
                if let Some(node) = parse_call(call_str) {
                    if !current.is_empty() {
                        segments.push(Node::StringLiteral(current.clone()));
                        current.clear();
                    }
                    segments.push(node);
                    // skip past the call
                    let skip = end - 1;
                    for _ in 0..skip {
                        chars.next();
                    }
                    continue;
                }
            }
        }
        current.push(ch);
    }

    if !current.is_empty() {
        segments.push(Node::StringLiteral(current));
    }

    segments
}

// finds the end index of a complete Z function call in a string
fn find_call_end(s: &str) -> Option<usize> {
    let brace_pos = s.find('{')?;
    // check it looks like Zname{
    let name_part = &s[1..brace_pos];
    if name_part.is_empty() || !name_part.chars().all(|c| c.is_alphanumeric()) {
        return None;
    }

    let mut depth = 0;
    for (i, ch) in s[brace_pos..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(brace_pos + i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn parse_call(s: &str) -> Option<Node> {
    let s = s.trim();
    if !s.starts_with('Z') {
        return None;
    }

    let brace_pos = s.find('{')?;
    let fn_name = s[1..brace_pos].to_string();
    if fn_name.is_empty() {
        return None;
    }

    let args_str = extract_args(&s[brace_pos + 1..])?;
    let arg_nodes = split_args(&args_str)
        .into_iter()
        .map(|a| parse_arg(&a))
        .collect();

    Some(Node::FunctionCall {
        name: fn_name,
        args: arg_nodes,
    })
}

fn parse_arg(s: &str) -> Node {
    let segments = parse_template(s);
    if segments.len() == 1 {
        segments.into_iter().next().unwrap()
    } else if segments.is_empty() {
        Node::StringLiteral(s.to_string())
    } else {
        Node::Concat(segments)
    }
}

fn extract_args(s: &str) -> Option<String> {
    let mut depth = 1;
    let mut end = 0;
    for (i, ch) in s.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth == 0 {
        Some(s[..end].to_string())
    } else {
        None
    }
}

fn split_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        // Escape sequences inside arguments
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                if next == '{' || next == ';' || next == '\\' {
                    chars.next();
                    current.push(next);
                    continue;
                }
            }
            current.push(ch);
            continue;
        }
        match ch {
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth -= 1;
                current.push(ch);
            }
            ';' if depth == 0 => {
                // Only trim if the result is non-empty after trimming,
                // otherwise preserve the raw value (e.g. a space separator).
                let trimmed = current.trim().to_string();
                let value = if trimmed.is_empty() && !current.is_empty() {
                    // The arg was whitespace-only — keep it as a single space
                    // so " " separators work correctly.
                    current.clone()
                } else {
                    trimmed
                };
                args.push(value);
                current = String::new();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    let trimmed = current.trim().to_string();
    let value = if trimmed.is_empty() && !current.is_empty() {
        current
    } else {
        trimmed
    };
    if !value.is_empty() {
        args.push(value);
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_calls_in_arg() {
        let node = parse_arg("Zf1{}Zf2{}");
        match node {
            Node::Concat(segments) => {
                assert_eq!(segments.len(), 2);
            }
            other => panic!("expected Concat, got {other:?}"),
        }
    }

    #[test]
    fn strips_inline_comment_outside_braces() {
        let node = parse_line("Hello! // this is a comment");
        assert!(matches!(node, Some(Node::StringLiteral(s)) if s == "Hello!"));
    }

    #[test]
    fn keeps_comment_marker_inside_braces() {
        let node = parse_line("Zf{arg1 // comment; arg2}").unwrap();
        match node {
            Node::FunctionCall { name, args } => {
                assert_eq!(name, "f");
                assert_eq!(args.len(), 2);
                match &args[0] {
                    Node::StringLiteral(s) => assert_eq!(s, "arg1 // comment"),
                    other => panic!("unexpected arg[0]: {other:?}"),
                }
                match &args[1] {
                    Node::StringLiteral(s) => assert_eq!(s, "arg2"),
                    other => panic!("unexpected arg[1]: {other:?}"),
                }
            }
            other => panic!("unexpected node: {other:?}"),
        }
    }

    #[test]
    fn strips_comment_after_braced_section() {
        let node = parse_line("Hello {keep // this} world // remove");
        assert!(matches!(
            node,
            Some(Node::StringLiteral(s)) if s == "Hello {keep // this} world"
        ));
    }
}
