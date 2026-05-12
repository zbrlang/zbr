use crate::context::FnOutput;

/// Parse a float argument, returning a clear error on failure.
pub fn parse_f64(s: &str, fn_name: &str, position: &str) -> Result<f64, FnOutput> {
    if s.is_empty() {
        return Err(FnOutput::error(fn_name, format!("{} cannot be empty", position)));
    }
    s.parse::<f64>().map_err(|_| {
        FnOutput::error(fn_name, format!("invalid number for {}: '{}'", position, s))
    })
}

/// Parse an integer argument, returning a clear error on failure.
pub fn parse_i64(s: &str, fn_name: &str, position: &str) -> Result<i64, FnOutput> {
    if s.is_empty() {
        return Err(FnOutput::error(fn_name, format!("{} cannot be empty", position)));
    }
    s.parse::<i64>().map_err(|_| {
        FnOutput::error(fn_name, format!("invalid integer for {}: '{}'", position, s))
    })
}
