use super::types::U256;
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

pub fn convert(value: U256, from: UnitType, to: UnitType) -> Option<U256> {
    if from == to {
        return Some(value);
    }
    if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
        return None;
    }
    match (find_conversion_factor(from), find_conversion_factor(to)) {
        (Ok(from), Ok(to)) => Some(value * from / to),
        _ => None,
    }
}

/// Finds conversion factor if applicable, otherwise return which
/// actual unit does not have a fixed conversion factor.
pub fn find_conversion_factor(u: UnitType) -> Result<U256, Error> {
    Ok(match u {
        UnitType::TIME(v) => match v {
            Time::SECOND => U256::from(1),
            Time::MINUTE => U256::from(60),
            Time::HOUR => U256::from(3600),
            Time::DAY => U256::from(86400),
            Time::WEEK => U256::from(604800),
            Time::MONTH => U256::from(2628000),
            Time::YEAR => U256::from(31536000),
        },
        UnitType::EVMGAS(v) => match v {
            EvmGas::WEI => U256::from(1),
            EvmGas::KILOWEI => U256::from(1e3 as u64),
            EvmGas::MEGAWEI => U256::from(1e6 as u64),
            EvmGas::GIGAWEI => U256::from(1e9 as u64),
            EvmGas::MICROETHER => U256::from(1e12 as u64),
            EvmGas::MILLIETHER => U256::from(1e15 as u64),
            EvmGas::ETHER => U256::from(1e18 as u64),
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
