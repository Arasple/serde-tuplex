use crate::internal::LenientValue;
use serde::{Deserialize, Deserializer};

/// Deserialize numeric values that accept both numbers and strings.
///
/// Use with `#[serde(deserialize_with = "lenient")]`.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_tuplex::lenient;
///
/// #[derive(Deserialize)]
/// struct Config {
///     #[serde(deserialize_with = "lenient")]
///     timeout: u64,
/// }
///
/// // Both work
/// let cfg1: Config = serde_json::from_str(r#"{"timeout": 3000}"#).unwrap();
/// let cfg2: Config = serde_json::from_str(r#"{"timeout": "3000"}"#).unwrap();
/// ```
pub fn lenient<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    LenientValue::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

/// Deserialize `Option<T>` with lenient parsing for numeric values.
///
/// **Important**: Must be combined with `#[serde(default)]` to handle missing fields.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_tuplex::lenient_option;
///
/// #[derive(Deserialize)]
/// struct User {
///     #[serde(deserialize_with = "lenient_option", default)]
///     age: Option<u32>,
/// }
///
/// // All work
/// let u1: User = serde_json::from_str(r#"{"age": 25}"#).unwrap();
/// let u2: User = serde_json::from_str(r#"{"age": "25"}"#).unwrap();
/// let u3: User = serde_json::from_str(r#"{"age": null}"#).unwrap();
/// let u4: User = serde_json::from_str(r#"{}"#).unwrap();
/// ```
pub fn lenient_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
{
    Option::<LenientValue>::deserialize(deserializer)?
        .map(|v| v.parse())
        .transpose()
        .map_err(serde::de::Error::custom)
}
