#![allow(dead_code)]
use alloy_core::primitives::U256;
use chrono::format;
use super::utils::vectorize_str;

pub enum ParseResult {
    Value(U256),
    String(String),
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

use alloy_dyn_abi::{DynSolType, DynSolValue};
use serde::{ser::SerializeSeq, Serialize, Serializer};

#[derive(Debug)]
pub struct Encodable(DynSolValue);

pub fn abi_parse_argument(r#type: &str, data: &str) -> Result<Encodable, String> {
    let data = if data.starts_with("[") && data.ends_with("]") {
        data.trim_matches(|c| c == '[' || c == ']')
    } else {
        data
    };
    let parsed = if data.starts_with("0x") && data.len() > 66 {
        let mut parsed = data[2..66].to_string();
        for i in (66..data.len()).step_by(64) {
            let end = std::cmp::min(i + 64, data.len() - i);
            parsed = format!("{}, {}", parsed, &data[i..i + end]);
        }
        format!("({})", parsed)
    } else {
        format!("({})", data)
    };
    gloo_console::log!("abi_parse_argument: {:?}", r#type);
    gloo_console::log!("abi_parse_argument: {:?}", data);
    match r#type.parse::<DynSolType>() {
        Ok(dyn_type) => dyn_type
            .coerce_str(&parsed)
            .map(Encodable)
            .map_err(|e| {
                gloo_console::log!(format!("Failed to parse data: {}", e));
                e.to_string()
            }),
        Err(e) => {
            gloo_console::log!(format!("Failed to parse type: {}", e));
            Err(format!("Failed to parse type: {}", e))},
    }
}

impl Serialize for Encodable {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            DynSolValue::Bool(v) => v.serialize(serializer),
            DynSolValue::Int(v, _) => v.serialize(serializer),
            DynSolValue::Uint(v, _) => v.serialize(serializer),
            DynSolValue::FixedBytes(v, _) => v.serialize(serializer),
            DynSolValue::Address(v) => v.serialize(serializer),
            DynSolValue::Function(_) => {
                gloo_console::log!("we only expect to parse function arguments and responses");
                unreachable!("we only expect to parse function arguments and responses")
            }
            DynSolValue::Bytes(v) => v.serialize(serializer),
            DynSolValue::String(v) => v.serialize(serializer),
            DynSolValue::Array(v)
            | DynSolValue::FixedArray(v)
            | DynSolValue::Tuple(v)
            // | DynSolValue:: { tuple: v, .. } => {
             => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for elem in v.iter() {
                    seq.serialize_element(&Encodable(elem.clone()))?;
                }
                seq.end()
            }
        }
    }
}