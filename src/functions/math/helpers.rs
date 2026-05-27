use crate::context::FnOutput;

/// Parse a float argument, returning a clear error on failure.
pub fn parse_f64(s: &str, fn_name: &str, arg_num: usize, arg_name: &str) -> Result<f64, FnOutput> {
    if s.is_empty() {
        return Err(FnOutput::error(fn_name, crate::error_messages::required(arg_num, arg_name)));
    }
    s.parse::<f64>().map_err(|_| {
        FnOutput::error(fn_name, crate::error_messages::expected_number(arg_num, arg_name, s))
    })
}

/// Parse an integer argument, returning a clear error on failure.
pub fn parse_i64(s: &str, fn_name: &str, arg_num: usize, arg_name: &str) -> Result<i64, FnOutput> {
    if s.is_empty() {
        return Err(FnOutput::error(fn_name, crate::error_messages::required(arg_num, arg_name)));
    }
    s.parse::<i64>().map_err(|_| {
        FnOutput::error(fn_name, crate::error_messages::expected_integer(arg_num, arg_name, s))
    })
}
