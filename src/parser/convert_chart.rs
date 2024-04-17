use super::utils::*;
use alloy_core::primitives::U256;
use gloo_console::log;
use std::io::Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Time {
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EvmGas {
    WEI,
    KILOWEI,
    MEGAWEI,
    GIGAWEI,
    MICROETHER,
    MILLIETHER,
    ETHER,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnitType {
    TIME(Time),
    EVMGAS(EvmGas),
}

pub fn convert(value: &str, from: UnitType, to: UnitType) -> Option<U256> {
    if from == to && !value.contains(".") {
        return Some(value.parse::<U256>().unwrap());
    }
    if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
        return None;
    }
    match (find_conversion_factor(from), find_conversion_factor(to)) {
        (Ok(from), Ok(to)) => {
            let from = U256::from(from);
            let to = U256::from(to);
            let mut float_iter = value.split(".");

            // process integer and fractional parts separetely
            let base_str = float_iter.next().unwrap_or("0");
            let frac_str = remove_trailing_zeros(float_iter.next().unwrap_or("0"));
            let frac_exp = U256::from(10).checked_pow(U256::from(frac_str.chars().count()))?;
            // merge integer and fractional parts (risk of precision loss here)
            let uint = format!("{base_str}{frac_str}").parse::<U256>().unwrap();
            uint.checked_mul(from)?
                .checked_div(to)?
                .checked_div(frac_exp)
        }
        _ => None,
    }
}

/// Finds conversion factor if applicable, otherwise return which
/// actual unit does not have a fixed conversion factor.
pub fn find_conversion_factor(u: UnitType) -> Result<u64, Error> {
    Ok(match u {
        UnitType::TIME(v) => match v {
            Time::SECOND => 1,
            Time::MINUTE => 60,
            Time::HOUR => 3600,
            Time::DAY => 86400,
            Time::WEEK => 604800,
            Time::MONTH => 2628000,
            Time::YEAR => 31536000,
        },
        UnitType::EVMGAS(v) => match v {
            EvmGas::WEI => 1,
            EvmGas::KILOWEI => 1e3 as u64,
            EvmGas::MEGAWEI => 1e6 as u64,
            EvmGas::GIGAWEI => 1e9 as u64,
            EvmGas::MICROETHER => 1e12 as u64,
            EvmGas::MILLIETHER => 1e15 as u64,
            EvmGas::ETHER => 1e18 as u64,
        },
    })
}

impl std::str::FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Time
            "TIME::SECOND" => Ok(UnitType::TIME(Time::SECOND)),
            "TIME::MINUTE" => Ok(UnitType::TIME(Time::MINUTE)),
            "TIME::HOUR" => Ok(UnitType::TIME(Time::HOUR)),
            "TIME::DAY" => Ok(UnitType::TIME(Time::DAY)),
            "TIME::WEEK" => Ok(UnitType::TIME(Time::WEEK)),
            "TIME::MONTH" => Ok(UnitType::TIME(Time::MONTH)),
            "TIME::YEAR" => Ok(UnitType::TIME(Time::YEAR)),

            // EVM Gas
            "EVMGAS::WEI" => Ok(UnitType::EVMGAS(EvmGas::WEI)),
            "EVMGAS::KILOWEI" => Ok(UnitType::EVMGAS(EvmGas::KILOWEI)),
            "EVMGAS::MEGAWEI" => Ok(UnitType::EVMGAS(EvmGas::MEGAWEI)),
            "EVMGAS::GIGAWEI" => Ok(UnitType::EVMGAS(EvmGas::GIGAWEI)),
            "EVMGAS::MICROETHER" => Ok(UnitType::EVMGAS(EvmGas::MICROETHER)),
            "EVMGAS::MILLIETHER" => Ok(UnitType::EVMGAS(EvmGas::MILLIETHER)),
            "EVMGAS::ETHER" => Ok(UnitType::EVMGAS(EvmGas::ETHER)),

            // Otherwise
            _ => Err(format!("'{}' is not a valid value for UnitType", s)),
        }
    }
}
