use crate::context::{DiscordContext, FnOutput};

pub fn run(args: Vec<String>, _ctx: &DiscordContext) -> FnOutput {
    let list_str = &args[0];
    let separator = &args[1];
    let operation = args[2].to_lowercase();

    let mut numbers: Vec<f64> = Vec::new();
    for (i, s) in list_str.split(separator).enumerate() {
        let s = s.trim();
        if s.is_empty() { continue; }
        match s.parse::<f64>() {
            Ok(n) => numbers.push(n),
            Err(_) => return FnOutput::error("listMath", format!("Invalid number at index {}: {}", i + 1, s)),
        }
    }

    if numbers.is_empty() {
        return FnOutput::Text("0".to_string());
    }

    let result = match operation.as_str() {
        "sum" => numbers.iter().sum(),
        "avg" => numbers.iter().sum::<f64>() / numbers.len() as f64,
        "min" => *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        "max" => *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        "median" => {
            numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = numbers.len() / 2;
            if numbers.len() % 2 == 0 {
                (numbers[mid - 1] + numbers[mid]) / 2.0
            } else {
                numbers[mid]
            }
        },
        "product" => numbers.iter().product(),
        _ => return FnOutput::error("listMath", format!("Invalid operation: {}", operation)),
    };

    FnOutput::Text(result.to_string())
}
