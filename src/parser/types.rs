pub use alloy_core::primitives::U256;
use alloy_core::primitives::*;

pub enum FloatOrU256 {
    Float(f64),
    U256(U256),
    NAN,
}

impl FloatOrU256 {
    pub fn is_f64(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn is_u256(&self) -> bool {
        matches!(self, Self::U256(_))
    }

    pub fn is_nan(&self) -> bool {
        matches!(self, Self::NAN)
    }
}

impl TryFrom<FloatOrU256> for f64 {
    type Error = &'static str;

    fn try_from(value: FloatOrU256) -> Result<Self, Self::Error> {
        match value {
            FloatOrU256::Float(f) => Ok(f),
            FloatOrU256::U256(u) => {
                if u <= U256::from(u64::MAX) {
                    Ok(u.saturating_to::<u64>() as f64)
                } else {
                    Err("Cannot convert U256 to f64 without loss of precision")
                }
            }
            //.to_f64()
            //.ok_or("Cannot convert U256 to f64 without loss of precision"),
            FloatOrU256::NAN => Err("Cannot convert U256 to f64 without loss of precision"),
        }
    }
}

impl From<f64> for FloatOrU256 {
    fn from(f: f64) -> Self {
        FloatOrU256::Float(f)
    }
}

impl From<U256> for FloatOrU256 {
    fn from(u: U256) -> Self {
        FloatOrU256::U256(u)
    }
}
