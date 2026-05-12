use crate::context::DiscordContext;
use serde_json::Value;

/// Navigate into a JSON value following a slice of string keys.
/// Object keys are matched by name; array keys must be numeric (0-based).
/// Returns a mutable reference path — since we can't return &mut through
/// multiple levels easily, we use a clone-and-replace pattern instead.
/// This helper returns a cloned sub-value at the given path, or None.
pub fn get_at_path<'a>(root: &'a Value, keys: &[String]) -> Option<&'a Value> {
    let mut cur = root;
    for key in keys {
        cur = match cur {
            Value::Object(map) => map.get(key.as_str())?,
            Value::Array(arr) => {
                let i: usize = key.parse().ok()?;
                arr.get(i)?
            }
            _ => return None,
        };
    }
    Some(cur)
}

/// Set a value at the given path, creating intermediate objects as needed.
/// Returns false if a path segment exists but is not an object/array.
pub fn set_at_path(root: &mut Value, keys: &[String], value: Value) -> bool {
    if keys.is_empty() {
        *root = value;
        return true;
    }
    let key = &keys[0];
    let rest = &keys[1..];

    // Ensure root is an object if we need to descend
    if !root.is_object() {
        *root = Value::Object(serde_json::Map::new());
    }

    if let Value::Object(map) = root {
        if rest.is_empty() {
            map.insert(key.clone(), value);
            return true;
        }
        let child = map.entry(key.clone()).or_insert(Value::Object(serde_json::Map::new()));
        set_at_path(child, rest, value)
    } else {
        false
    }
}

/// Remove a key at the given path. Returns true if something was removed.
pub fn unset_at_path(root: &mut Value, keys: &[String]) -> bool {
    if keys.is_empty() {
        return false;
    }
    if keys.len() == 1 {
        if let Value::Object(map) = root {
            return map.remove(keys[0].as_str()).is_some();
        }
        return false;
    }
    let key = &keys[0];
    let rest = &keys[1..];
    if let Value::Object(map) = root {
        if let Some(child) = map.get_mut(key.as_str()) {
            return unset_at_path(child, rest);
        }
    }
    false
}

/// Get a mutable reference to the value at the given path.
pub fn get_mut_at_path<'a>(root: &'a mut Value, keys: &[String]) -> Option<&'a mut Value> {
    let mut cur = root;
    for key in keys {
        cur = match cur {
            Value::Object(map) => map.get_mut(key.as_str())?,
            Value::Array(arr) => {
                let i: usize = key.parse().ok()?;
                arr.get_mut(i)?
            }
            _ => return None,
        };
    }
    Some(cur)
}

/// Lock the json_object, run a closure with &mut Option<Value>, return result.
pub fn with_json<F, T>(ctx: &DiscordContext, f: F) -> T
where
    F: FnOnce(&mut Option<Value>) -> T,
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { f(&mut *ctx.json_object.lock().await) })
    })
}

/// Parse a string value into the most appropriate JSON type.
/// "true"/"false" → Bool, numeric strings → Number, everything else → String.
pub fn infer_value(s: &str) -> Value {
    if s == "true" {
        return Value::Bool(true);
    }
    if s == "false" {
        return Value::Bool(false);
    }
    if s == "null" {
        return Value::Null;
    }
    if let Ok(i) = s.parse::<i64>() {
        return Value::Number(i.into());
    }
    if let Ok(f) = s.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return Value::Number(n);
        }
    }
    Value::String(s.to_string())
}
