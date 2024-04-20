use alloy_core::primitives::U256;

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
