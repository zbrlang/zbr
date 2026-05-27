use crate::context::{DiscordContext, FnOutput};
use super::helpers::{parse_f64, parse_i64};

// Zsort{n1;n2;...;direction;return_amount;separator}
// The last 3 args are always direction, return_amount, separator.
// Everything before them is the list of numbers (minimum 1 number = 4 args total,
// but we require at least 2 numbers so min_args = 5).
pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    if args.len() < 5 {
        return FnOutput::error("sort", crate::error_messages::too_few_args(5, args.len()));
    }

    let separator  = args.last().unwrap().clone();
    let ret_amount = args[args.len() - 2].clone();
    let direction  = args[args.len() - 3].clone();
    let number_args = &args[..args.len() - 3];

    // Parse direction
    let ascending = match direction.as_str() {
        "asc"  => true,
        "desc" => false,
        other  => return FnOutput::error("sort", crate::error_messages::expected_choice(args.len() - 2, "direction", "asc, desc", other)),
    };

    // Parse return amount
    let ret_n = match parse_i64(&ret_amount, "sort", args.len() - 2, "return amount") {
        Ok(v) => v, Err(e) => return e,
    };
    if ret_n < -1 || ret_n == 0 {
        return FnOutput::error("sort", crate::error_messages::out_of_range(args.len() - 1, "return amount", -1, i64::MAX, ret_n));
    }

    // Parse numbers
    let mut numbers: Vec<f64> = Vec::with_capacity(number_args.len());
    for (i, arg) in number_args.iter().enumerate() {
        let n = match parse_f64(arg, "sort", i + 1, "value") {
            Ok(v) => v, Err(e) => return e,
        };
        numbers.push(n);
    }

    // Sort
    if ascending {
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        numbers.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Truncate to return amount
    let take = if ret_n == -1 { numbers.len() } else { ret_n as usize };
    numbers.truncate(take);

    // Format each number (drop .0 for whole numbers)
    let parts: Vec<String> = numbers.iter().map(|&n| {
        if n.fract() == 0.0 && n.abs() < 1e15 {
            format!("{}", n as i64)
        } else {
            format!("{}", n)
        }
    }).collect();

    FnOutput::Text(parts.join(&separator))
}
