#![allow(dead_code)]
use alloy_core::primitives::{B256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseResult {
    Value(U256),
    String(String),
    Json(serde_json::Value),
    NAN,
}

impl ParseResult {
    pub fn is_str(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_address(&self) -> bool {
        match self {
            Self::String(s) => s.starts_with("0x") && s.len() == 42,
            _ => false,
        }
    }

    pub fn is_u256(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn is_nan(&self) -> bool {
        matches!(self, Self::NAN)
    }
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub fn get_json(&self) -> Option<serde_json::Value> {
        match self {
            Self::Json(j) => Some(j.to_owned()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.to_owned()),
            _ => None,
        }
    }

    pub fn to_hex_string(&self, full_evm_word: bool) -> String {
        match self {
            ParseResult::Value(u) => {
                let hex: B256 = (*u).into();
                let hex_str = hex.to_string();
                // When `full_evm_word` is `false`, trim the leading zeros from the hex representation
                if !full_evm_word {
                    let hex_formatted = format!("0x{}", hex_str[2..].trim_start_matches("0"));
                    if hex_formatted == "0x" {
                        "0x0".to_string()
                    } else {
                        hex_formatted
                    }
                } else {
                    hex_str
                }
            }
            ParseResult::String(s) => s.to_string(),
            ParseResult::Json(j) => j.to_string(),
            ParseResult::NAN => "-".to_string(),
        }
    }
}

impl Default for ParseResult {
    fn default() -> Self {
        ParseResult::NAN
    }
}

impl From<String> for ParseResult {
    fn from(s: String) -> Self {
        ParseResult::String(s)
    }
}

impl From<&str> for ParseResult {
    fn from(s: &str) -> Self {
        ParseResult::String(s.to_string())
    }
}

impl From<U256> for ParseResult {
    fn from(u: U256) -> Self {
        ParseResult::Value(u)
    }
}

impl From<i64> for ParseResult {
    fn from(i: i64) -> Self {
        let u = U256::from(i);
        ParseResult::Value(u)
    }
}

impl From<Option<U256>> for ParseResult {
    fn from(u: Option<U256>) -> Self {
        match u {
            Some(u) => ParseResult::Value(u),
            None => ParseResult::NAN,
        }
    }
}

impl From<Option<String>> for ParseResult {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => ParseResult::String(s),
            None => ParseResult::NAN,
        }
    }
}

impl From<serde_json::Value> for ParseResult {
    fn from(v: serde_json::Value) -> Self {
        ParseResult::Json(v)
    }
}

impl ToString for ParseResult {
    fn to_string(&self) -> String {
        match self {
            ParseResult::Value(u) => u.to_string(),
            ParseResult::String(s) => s.to_string(),
            ParseResult::Json(j) => j.to_string(),
            ParseResult::NAN => "-".to_string(),
        }
    }
}
