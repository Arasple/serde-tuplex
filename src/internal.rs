//! Internal types for lenient parsing. Not part of public API.

use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

/// Captures numeric values in either string or native format.
#[derive(Debug)]
pub enum LenientValue {
    String(String),
    I64(i64),
    U64(u64),
    F64(f64),
}

impl LenientValue {
    /// Parse to target type via `FromStr`.
    pub fn parse<T>(&self) -> Result<T, String>
    where
        T: FromStr,
        T::Err: fmt::Display,
    {
        match self {
            LenientValue::String(s) => s.parse().map_err(|e| format!("{}", e)),
            LenientValue::I64(v) => v.to_string().parse().map_err(|e| format!("{}", e)),
            LenientValue::U64(v) => v.to_string().parse().map_err(|e| format!("{}", e)),
            LenientValue::F64(v) => v.to_string().parse().map_err(|e| format!("{}", e)),
        }
    }
}

impl<'de> Deserialize<'de> for LenientValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LenientValueVisitor;

        impl<'de> serde::de::Visitor<'de> for LenientValueVisitor {
            type Value = LenientValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LenientValue::String(value.to_string()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LenientValue::I64(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LenientValue::U64(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LenientValue::F64(value))
            }
        }

        deserializer.deserialize_any(LenientValueVisitor)
    }
}

/// Wrapper for `Option<LenientValue>`.
#[derive(Debug)]
pub struct OptionalLenientValue(Option<LenientValue>);

impl OptionalLenientValue {
    pub fn into_option(self) -> Option<LenientValue> {
        self.0
    }
}

impl<'de> Deserialize<'de> for OptionalLenientValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OptionalLenientValue(Option::<LenientValue>::deserialize(
            deserializer,
        )?))
    }
}
