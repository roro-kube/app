// Port value type
//
// This module defines the PortValue type and its serialization.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Port value that can be either a numeric port or a named port string
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortValue {
    /// Numeric port number
    Numeric(u16),
    /// Named port (e.g., "prometheus", "http")
    Named(String),
}

impl Serialize for PortValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PortValue::Numeric(n) => serializer.serialize_u16(*n),
            PortValue::Named(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for PortValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    match u16::try_from(u) {
                        Ok(port) => Ok(PortValue::Numeric(port)),
                        Err(_) => Err(serde::de::Error::custom(format!(
                            "Port number {} exceeds maximum value {}",
                            u,
                            u16::MAX
                        ))),
                    }
                } else {
                    Err(serde::de::Error::custom("Port number must be a valid u16"))
                }
            }
            Value::String(s) => Ok(PortValue::Named(s)),
            _ => Err(serde::de::Error::custom(
                "Port must be either a number or a string",
            )),
        }
    }
}
