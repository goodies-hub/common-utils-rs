use std::env;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Environment variable `{0}` is not set")]
    Missing(String),

    #[error("Failed to parse environment variable `{key}`: {value}")]
    ParseError { key: String, value: String },
}

pub fn get_required(key: &str) -> Result<String, EnvError> {
    env::var(key).map_err(|_| EnvError::Missing(key.to_string()))
}

pub fn get_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_parsed<T>(key: &str) -> Result<T, EnvError> where T: FromStr, {
    let value = get_required(key)?;
    value.parse::<T>().map_err(|_| EnvError::ParseError {
        key: key.to_string(),
        value,
    })
}

pub fn get_parsed_or_default<T>(key: &str, default: T) -> T where T: FromStr, {
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}

pub fn get_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(val) => matches!(val.to_lowercase().as_str(), "true" | "1" | "yes" | "on"),
        Err(_) => default,
    }
}

pub fn get_list(key: &str) -> Result<Vec<String>, EnvError> {
    let value = get_required(key)?;
    Ok(value.split(',').map(|s| s.trim().to_string()).collect())
}

/// Parse memory size (e.g. 10MB, 512KB, 1GB)
pub fn parse_memory_size(input: &str) -> Result<usize, EnvError> {
    let input = input.trim().to_uppercase();

    let (num_part, multiplier) = if input.ends_with("KB") {
        (&input[..input.len() - 2], 1024)
    } else if input.ends_with("MB") {
        (&input[..input.len() - 2], 1024 * 1024)
    } else if input.ends_with("GB") {
        (&input[..input.len() - 2], 1024 * 1024 * 1024)
    } else {
        (input.as_str(), 1)
    };

    let number: usize = num_part.parse().map_err(|_| EnvError::ParseError {
        key: "memory_size".to_string(),
        value: input.clone(),
    })?;

    Ok(number * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn reset_env(key: &str) {
        unsafe {
            env::remove_var(key);
        }
    }

    #[test]
    fn test_get_required_success() {
        unsafe {
            env::set_var("REQUIRED_KEY", "value");
            assert_eq!(get_required("REQUIRED_KEY").unwrap(), "value");
            reset_env("REQUIRED_KEY");
        }
    }

    #[test]
    fn test_get_required_missing() {
        reset_env("MISSING_KEY");
        let result = get_required("MISSING_KEY");
        assert!(matches!(result, Err(EnvError::Missing(_))));
    }

    #[test]
    fn test_get_or_default() {
        reset_env("DEFAULT_KEY");
        assert_eq!(get_or_default("DEFAULT_KEY", "default"), "default");
        unsafe {
            env::set_var("DEFAULT_KEY", "value");
            assert_eq!(get_or_default("DEFAULT_KEY", "default"), "value");
            reset_env("DEFAULT_KEY");
        }
    }

    #[test]
    fn test_get_parsed_success() {
        unsafe {
            env::set_var("PARSE_KEY", "42");
            let result: i32 = get_parsed("PARSE_KEY").unwrap();
            assert_eq!(result, 42);
            reset_env("PARSE_KEY");
        }
    }

    #[test]
    fn test_get_parsed_failure() {
        unsafe {
            env::set_var("PARSE_BAD", "abc");
            let result: Result<i32, _> = get_parsed("PARSE_BAD");
            assert!(matches!(result, Err(EnvError::ParseError { .. })));
            reset_env("PARSE_BAD");
        }
    }

    #[test]
    fn test_get_parsed_or_default() {
        reset_env("PARSE_DEFAULT");
        let value: u64 = get_parsed_or_default("PARSE_DEFAULT", 100);
        assert_eq!(value, 100);
        unsafe {
            env::set_var("PARSE_DEFAULT", "200");
            let value: u64 = get_parsed_or_default("PARSE_DEFAULT", 100);
            assert_eq!(value, 200);
            reset_env("PARSE_DEFAULT");
        }
    }

    #[test]
    fn test_get_bool_true_values() {
        unsafe {
            let trues = ["true", "1", "yes", "on"];
            for &v in &trues {
                env::set_var("BOOL_KEY", v);
                assert!(get_bool("BOOL_KEY", false));
            }
            reset_env("BOOL_KEY");
        }
    }

    #[test]
    fn test_get_bool_false_values() {
        unsafe {
            let falses = ["false", "0", "no", "off", "random"];
            for &v in &falses {
                env::set_var("BOOL_KEY", v);
                assert!(!get_bool("BOOL_KEY", false));
            }
            reset_env("BOOL_KEY");
        }
    }

    #[test]
    fn test_get_bool_default() {
        reset_env("BOOL_DEFAULT");
        assert_eq!(get_bool("BOOL_DEFAULT", true), true);
        assert_eq!(get_bool("BOOL_DEFAULT", false), false);
    }

    #[test]
    fn test_get_list_success() {
        unsafe {
            env::set_var("LIST_KEY", "a, b ,c");
            let list = get_list("LIST_KEY").unwrap();
            assert_eq!(list, vec!["a", "b", "c"]);
            reset_env("LIST_KEY");
        }
    }

    #[test]
    fn test_get_list_missing() {
        reset_env("LIST_MISSING");
        let list = get_list("LIST_MISSING");
        assert!(matches!(list, Err(EnvError::Missing(_))));
    }

    #[test]
    fn test_parse_memory_size_kb_mb_gb_and_plain() {
        assert_eq!(parse_memory_size("1KB").unwrap(), 1024);
        assert_eq!(parse_memory_size("1MB").unwrap(), 1024 * 1024);
        assert_eq!(parse_memory_size("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_memory_size("123").unwrap(), 123);
    }

    #[test]
    fn test_parse_memory_size_invalid() {
        let result = parse_memory_size("abcMB");
        assert!(matches!(result, Err(EnvError::ParseError { .. })));
    }
}
