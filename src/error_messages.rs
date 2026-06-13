/// Centralized error message formatting for ZBR functions.
/// Format: `Line N: Z{function} - {message}`

pub fn too_few_args(expected: usize, got: usize) -> String {
    format!(
        "expected at least {} argument{}, got {}",
        expected,
        if expected == 1 {
            ""
        } else {
            "s"
        },
        got
    )
}
pub fn too_many_args(expected: usize, got: usize) -> String {
    format!(
        "expected at most {} argument{}, got {}",
        expected,
        if expected == 1 {
            ""
        } else {
            "s"
        },
        got
    )
}
pub fn exact_args(expected: usize, got: usize) -> String {
    format!(
        "expected exactly {} argument{}, got {}",
        expected,
        if expected == 1 {
            ""
        } else {
            "s"
        },
        got
    )
}

// Required but empty
pub fn required(arg_num: usize, arg_name: &str) -> String {
    format!("argument {} ({}) is required", arg_num, arg_name)
}

// Type errors
pub fn expected_snowflake(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected snowflake ID in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_number(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected number in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_integer(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected integer in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_boolean(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected true or false in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_url(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected URL in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_hex_color(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!("expected hex color in argument {} ({}), got \"{}\"", arg_num, arg_name, got)
}
pub fn expected_duration(arg_num: usize, arg_name: &str, got: &str) -> String {
    format!(
        "expected duration (e.g. 30s, 1m, 2h) in argument {} ({}), got \"{}\"",
        arg_num,
        arg_name,
        got
    )
}

// Enum / choice errors
pub fn expected_choice(arg_num: usize, arg_name: &str, choices: &str, got: &str) -> String {
    format!("expected one of ({}) in argument {} ({}), got \"{}\"", choices, arg_num, arg_name, got)
}

// Range errors
pub fn out_of_range(arg_num: usize, arg_name: &str, min: i64, max: i64, got: i64) -> String {
    format!("argument {} ({}) must be between {} and {}, got {}", arg_num, arg_name, min, max, got)
}
pub fn must_be_positive(arg_num: usize, arg_name: &str, got: i64) -> String {
    format!("argument {} ({}) must be a positive number, got {}", arg_num, arg_name, got)
}
pub fn too_long(arg_num: usize, arg_name: &str, max: usize, got: usize) -> String {
    format!("argument {} ({}) must be at most {} characters, got {}", arg_num, arg_name, max, got)
}

// Dependency / state errors
pub fn requires_first(dependency: &str) -> String {
    format!("requires {} to be called first", dependency)
}
pub fn requires_set_first(field: &str) -> String {
    format!("requires {} to be set first", field)
}
pub fn not_in_guild() -> String {
    "requires guild context".to_string()
}
pub fn not_found(resource: &str, identifier: &str) -> String {
    format!("{} \"{}\" not found", resource, identifier)
}
pub fn action_failed(action: &str) -> String {
    format!("failed to {}", action)
}
pub fn action_failed_reason(action: &str, reason: &str) -> String {
    format!("failed to {} ({})", action, reason)
}

// Setup / availability
pub fn not_available(component: &str) -> String {
    format!("{} is not available", component)
}

// Internal errors
pub fn internal_error(msg: &str) -> String {
    format!("internal error: {}", msg)
}

// Permission errors
pub fn unknown_permission(got: &str) -> String {
    format!("unknown permission \"{}\" — see docs for valid permission names", got)
}
pub fn missing_permission(perm: &str) -> String {
    format!("missing required permission: {}", perm)
}
