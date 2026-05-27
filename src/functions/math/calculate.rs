use crate::context::{DiscordContext, FnOutput};

/// Recursive-descent expression evaluator.
/// Supports: + - * / % ** () and unary minus.
/// Operator precedence (low → high):
///   1. + -
///   2. * / %
///   3. ** (right-associative)
///   4. unary -
///   5. atoms: numbers, parenthesised expressions

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let expr = args.get(0).cloned().unwrap_or_default();
    if expr.is_empty() {
        return FnOutput::error("calculate", crate::error_messages::required(1, "expression"));
    }
    let input: Vec<char> = expr.chars().collect();
    let mut pos = 0usize;
    match parse_expr(&input, &mut pos) {
        Ok(val) => {
            // Skip trailing whitespace and make sure we consumed everything
            skip_ws(&input, &mut pos);
            if pos < input.len() {
                return FnOutput::error(
                    "calculate",
                    format!("unexpected character '{}' in expression", input[pos]),
                );
            }
            // Format: drop ".0" suffix for whole numbers
            let s = if val.fract() == 0.0 && val.abs() < 1e15 {
                format!("{}", val as i64)
            } else {
                format!("{}", val)
            };
            FnOutput::Text(s)
        }
        Err(msg) => FnOutput::error("calculate", msg),
    }
}

// ── Parser ────────────────────────────────────────────────────────────────────

fn skip_ws(input: &[char], pos: &mut usize) {
    while *pos < input.len() && input[*pos].is_whitespace() {
        *pos += 1;
    }
}

/// additive: expr ('+' | '-') expr ...
fn parse_expr(input: &[char], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_term(input, pos)?;
    loop {
        skip_ws(input, pos);
        if *pos >= input.len() { break; }
        match input[*pos] {
            '+' => { *pos += 1; left += parse_term(input, pos)?; }
            '-' => { *pos += 1; left -= parse_term(input, pos)?; }
            _   => break,
        }
    }
    Ok(left)
}

/// multiplicative: factor ('*' | '/' | '%') factor ...
fn parse_term(input: &[char], pos: &mut usize) -> Result<f64, String> {
    let mut left = parse_power(input, pos)?;
    loop {
        skip_ws(input, pos);
        if *pos >= input.len() { break; }
        // Check for ** before consuming a single *
        if *pos + 1 < input.len() && input[*pos] == '*' && input[*pos + 1] == '*' { break; }
        match input[*pos] {
            '*' => { *pos += 1; left *= parse_power(input, pos)?; }
            '/' => {
                *pos += 1;
                let r = parse_power(input, pos)?;
                if r == 0.0 { return Err("cannot divide by zero".into()); }
                left /= r;
            }
            '%' => {
                *pos += 1;
                let r = parse_power(input, pos)?;
                if r == 0.0 { return Err("cannot modulo by zero".into()); }
                left %= r;
            }
            _ => break,
        }
    }
    Ok(left)
}

/// power: unary ('**' unary)* — right-associative
fn parse_power(input: &[char], pos: &mut usize) -> Result<f64, String> {
    let base = parse_unary(input, pos)?;
    skip_ws(input, pos);
    if *pos + 1 < input.len() && input[*pos] == '*' && input[*pos + 1] == '*' {
        *pos += 2;
        let exp = parse_power(input, pos)?; // right-associative: recurse
        Ok(base.powf(exp))
    } else {
        Ok(base)
    }
}

/// unary: '-' atom | atom
fn parse_unary(input: &[char], pos: &mut usize) -> Result<f64, String> {
    skip_ws(input, pos);
    if *pos < input.len() && input[*pos] == '-' {
        *pos += 1;
        Ok(-parse_atom(input, pos)?)
    } else {
        parse_atom(input, pos)
    }
}

/// atom: number | '(' expr ')'
fn parse_atom(input: &[char], pos: &mut usize) -> Result<f64, String> {
    skip_ws(input, pos);
    if *pos >= input.len() {
        return Err("unexpected end of expression".into());
    }
    if input[*pos] == '(' {
        *pos += 1;
        let val = parse_expr(input, pos)?;
        skip_ws(input, pos);
        if *pos >= input.len() || input[*pos] != ')' {
            return Err("missing closing parenthesis".into());
        }
        *pos += 1;
        return Ok(val);
    }
    parse_number(input, pos)
}

fn parse_number(input: &[char], pos: &mut usize) -> Result<f64, String> {
    skip_ws(input, pos);
    let start = *pos;
    // optional sign already handled by parse_unary; just collect digits/dot/e
    while *pos < input.len() && (input[*pos].is_ascii_digit() || input[*pos] == '.' || input[*pos] == 'e' || input[*pos] == 'E') {
        *pos += 1;
        // allow e+/e- in scientific notation
        if *pos < input.len() && (input[*pos - 1] == 'e' || input[*pos - 1] == 'E') {
            if input[*pos] == '+' || input[*pos] == '-' { *pos += 1; }
        }
    }
    if *pos == start {
        return Err(format!("expected a number, got '{}'", input.get(*pos).copied().unwrap_or('?')));
    }
    let s: String = input[start..*pos].iter().collect();
    s.parse::<f64>().map_err(|_| format!("invalid number: '{}'", s))
}
