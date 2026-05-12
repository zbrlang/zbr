/// Evaluate a condition string like "a==b", "x>5 && y!=0", "name contains hello".
/// Returns true/false or an error string.
///
/// Supported operators: == != > < >= <= contains startsWith endsWith
/// Supported combinators: && (all must be true), || (any must be true)
/// Mixing && and || in one condition is not allowed.
pub fn eval_condition(condition: &str) -> Result<bool, String> {
    let cond = condition.trim();

    // Detect combinator — must be consistent (all && or all ||)
    let has_and = cond.contains(" && ");
    let has_or = cond.contains(" || ");

    if has_and && has_or {
        return Err("cannot mix && and || in one condition — use nested Zif instead".to_string());
    }

    if has_and {
        let parts: Vec<&str> = cond.split(" && ").collect();
        for part in parts {
            if !eval_single(part.trim())? {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    if has_or {
        let parts: Vec<&str> = cond.split(" || ").collect();
        for part in parts {
            if eval_single(part.trim())? {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    eval_single(cond)
}

/// Evaluate a single comparison expression: "left op right"
fn eval_single(expr: &str) -> Result<bool, String> {
    // Try operators longest-first to avoid >= being parsed as >
    const OPS: &[&str] = &[">=", "<=", "!=", "==", ">", "<", " contains ", " startsWith ", " endsWith "];

    for op in OPS {
        if let Some(pos) = expr.find(op) {
            let left = expr[..pos].trim();
            let right = expr[pos + op.len()..].trim();
            return Ok(apply_op(left, right, op.trim()));
        }
    }

    // No operator found — treat as a boolean value ("true"/"1" = true)
    Ok(expr == "true" || expr == "1")
}

fn apply_op(left: &str, right: &str, op: &str) -> bool {
    match op {
        "==" => left == right,
        "!=" => left != right,
        "contains" => left.contains(right),
        "startsWith" => left.starts_with(right),
        "endsWith" => left.ends_with(right),
        ">" => compare_numeric(left, right, |a, b| a > b),
        "<" => compare_numeric(left, right, |a, b| a < b),
        ">=" => compare_numeric(left, right, |a, b| a >= b),
        "<=" => compare_numeric(left, right, |a, b| a <= b),
        _ => false,
    }
}

fn compare_numeric(left: &str, right: &str, f: impl Fn(f64, f64) -> bool) -> bool {
    match (left.parse::<f64>(), right.parse::<f64>()) {
        (Ok(a), Ok(b)) => f(a, b),
        // Fall back to lexicographic comparison if not numeric
        _ => f(left.len() as f64, right.len() as f64),
    }
}
