use std::io::Error;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Time {
    NANOSECOND,
    MICROSECOND,
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DigitalInfo {
    BIT,
    BYTE,
    KILOBIT,
    KILOBYTE,
    MEGABIT,
    MEGABYTE,
    GIGABIT,
    GIGABYTE,
    TERABIT,
    TERABYTE,
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
    DIGITALINFO(DigitalInfo),
    EVMGAS(EvmGas),
}

pub fn convert(value: f64, from: UnitType, to: UnitType) -> f64 {
    println!("value: {}", value);
    println!("from: {:?}", from);
    println!("to: {:?}", to);
    if from == to {
        return value;
    }
    if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
        return f64::NAN;
    }
    match (find_conversion_factor(from), find_conversion_factor(to)) {
        (Ok(from), Ok(to)) => {
            println!("from: {}", from);
            println!("to: {}", to);
            value * from / to
        }
        _ => f64::NAN,
    }
}

/// Finds conversion factor if applicable, otherwise return which
/// actual unit does not have a fixed conversion factor.
pub fn find_conversion_factor(u: UnitType) -> Result<f64, Error> {
    Ok(match u {
        UnitType::TIME(v) => match v {
            Time::NANOSECOND => 1e-9,
            Time::MICROSECOND => 1e-6,
            Time::MILLISECOND => 0.001,
            Time::SECOND => 1_f64,
            Time::MINUTE => 60_f64,
            Time::HOUR => 3600_f64,
            Time::DAY => 86400_f64,
            Time::WEEK => 604800_f64,
            Time::MONTH => 2592000_f64,
            Time::YEAR => 31536000_f64,
        },
        UnitType::DIGITALINFO(v) => match v {
            DigitalInfo::BIT => 0.00012207,
            DigitalInfo::BYTE => 0.000976563,
            DigitalInfo::KILOBIT => 0.125,
            DigitalInfo::KILOBYTE => 1_f64,
            DigitalInfo::MEGABIT => 128_f64,
            DigitalInfo::MEGABYTE => 1024_f64,
            DigitalInfo::GIGABIT => 131072_f64,
            DigitalInfo::GIGABYTE => 1048576_f64,
            DigitalInfo::TERABIT => 134217728_f64,
            DigitalInfo::TERABYTE => 1073741824_f64,
        },
        UnitType::EVMGAS(v) => match v {
            EvmGas::WEI => 0.000000001,
            EvmGas::KILOWEI => 0.000001,
            EvmGas::MEGAWEI => 0.001,
            EvmGas::GIGAWEI => 1_f64,
            EvmGas::MICROETHER => 1000_f64,
            EvmGas::MILLIETHER => 1000000_f64,
            EvmGas::ETHER => 1000000000_f64,
        },
    })
}

impl std::str::FromStr for UnitType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Time
            "TIME::NANOSECOND" => Ok(UnitType::TIME(Time::NANOSECOND)),
            "TIME::MICROSECOND" => Ok(UnitType::TIME(Time::MICROSECOND)),
            "TIME::MILLISECOND" => Ok(UnitType::TIME(Time::MILLISECOND)),
            "TIME::SECOND" => Ok(UnitType::TIME(Time::SECOND)),
            "TIME::MINUTE" => Ok(UnitType::TIME(Time::MINUTE)),
            "TIME::HOUR" => Ok(UnitType::TIME(Time::HOUR)),
            "TIME::DAY" => Ok(UnitType::TIME(Time::DAY)),
            "TIME::WEEK" => Ok(UnitType::TIME(Time::WEEK)),
            "TIME::MONTH" => Ok(UnitType::TIME(Time::MONTH)),
            "TIME::YEAR" => Ok(UnitType::TIME(Time::YEAR)),

            // DigitalInfo
            "DIGITALINFO::BIT" => Ok(UnitType::DIGITALINFO(DigitalInfo::BIT)),
            "DIGITALINFO::BYTE" => Ok(UnitType::DIGITALINFO(DigitalInfo::BYTE)),
            "DIGITALINFO::KILOBIT" => Ok(UnitType::DIGITALINFO(DigitalInfo::KILOBIT)),
            "DIGITALINFO::KILOBYTE" => Ok(UnitType::DIGITALINFO(DigitalInfo::KILOBYTE)),
            "DIGITALINFO::MEGABIT" => Ok(UnitType::DIGITALINFO(DigitalInfo::MEGABIT)),
            "DIGITALINFO::MEGABYTE" => Ok(UnitType::DIGITALINFO(DigitalInfo::MEGABYTE)),
            "DIGITALINFO::GIGABIT" => Ok(UnitType::DIGITALINFO(DigitalInfo::GIGABIT)),
            "DIGITALINFO::GIGABYTE" => Ok(UnitType::DIGITALINFO(DigitalInfo::GIGABYTE)),
            "DIGITALINFO::TERABIT" => Ok(UnitType::DIGITALINFO(DigitalInfo::TERABIT)),
            "DIGITALINFO::TERABYTE" => Ok(UnitType::DIGITALINFO(DigitalInfo::TERABYTE)),

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
