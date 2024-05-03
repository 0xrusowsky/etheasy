#![allow(deprecated)]
mod convert_chart;
pub mod types;
pub mod utils;
use convert_chart::{convert, UnitType};
use types::*;
use utils::*;

use alloy_core::primitives::{
    utils::{format_ether, format_units, keccak256},
    B256, U256,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use gloo_console::log;
use pest::{
    iterators::{Pair, Pairs},
    prec_climber::*,
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct Calculator;

lazy_static::lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(modulus, Left),
            Operator::new(power, Right),
            Operator::new(rightShift, Right) | Operator::new(leftShift, Right),
        ])
    };
}

pub fn parse(input: &str) -> ParseResult {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(r) => eval(r, false),
        Err(_) => ParseResult::NAN,
    }
}

fn eval(expression: Pairs<Rule>, unchecked: bool) -> ParseResult {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::convert => {
                let mut i = pair.into_inner();
                let value = i.next().unwrap().as_str();
                let unit_type = i
                    .clone()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let from = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                let to = i
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_rule();
                if let (Ok(from), Ok(to)) = (
                    format!("{:?}::{:?}", unit_type, from).parse::<UnitType>(),
                    format!("{:?}::{:?}", unit_type, to).parse::<UnitType>(),
                ) {
                    convert(value, from, to).into()
                } else {
                    ParseResult::NAN
                }
            }
            Rule::function_val => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                if name == "unchecked" {
                    eval(i, true)
                } else {
                    match eval(i, unchecked) {
                        ParseResult::Value(value) => utility_fn_val(name, value),
                        _ => ParseResult::NAN,
                    }
                }
            }
            Rule::function_str => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                if name.starts_with("unix") {
                    match parse_encoded_utility_fn(name, "unix") {
                        Some(value) => utility_fn_str("unix", &value),
                        None => ParseResult::NAN,
                    }
                } else {
                    let value = i.next().unwrap().as_str();
                    utility_fn_str(name, value)
                }
            }
            Rule::function_args => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                utility_fn_args(name, i, unchecked)
            }
            Rule::now => U256::from(Utc::now().timestamp()).into(),
            Rule::addr_zero => String::from("0x0000000000000000000000000000000000000000").into(),
            Rule::max_uint => "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .ok()
                .into(),
            Rule::num => {
                let value_str = pair.as_str().trim();
                if value_str.contains("e") {
                    scientific_to_u256(value_str).into()
                } else {
                    value_str.parse::<U256>().ok().into()
                }
            }
            Rule::hex => {
                let pref_hex = pair.as_str().trim();
                let hex = if pref_hex.starts_with("0x") {
                    &pref_hex[2..]
                } else {
                    &pref_hex[1..]
                };
                U256::from_str_radix(hex, 16).ok().into()
            }
            Rule::bin => {
                let pref_bin = pair.as_str().trim();
                let bin = if pref_bin.starts_with("0b") {
                    &pref_bin[2..]
                } else {
                    &pref_bin[1..]
                };
                U256::from_str_radix(bin, 2).ok().into()
            }
            Rule::quote => trim_quotes(pair.as_str()).into(),
            Rule::array => trim_quotes(pair.as_str()).into(),
            Rule::expr => eval(pair.into_inner(), unchecked),
            _ => ParseResult::NAN,
        },
        |lhs: ParseResult, op: Pair<Rule>, rhs: ParseResult| {
            let (lhs, rhs) = match (lhs, rhs) {
                (ParseResult::Value(lhs), ParseResult::Value(rhs)) => (lhs, rhs),
                _ => return ParseResult::NAN,
            };
            match op.as_rule() {
                Rule::add => {
                    if unchecked {
                        let (result, _) = lhs.overflowing_add(rhs);
                        result.into()
                    } else {
                        lhs.checked_add(rhs).into()
                    }
                }
                Rule::subtract => {
                    if unchecked {
                        let (result, _) = lhs.overflowing_sub(rhs);
                        result.into()
                    } else {
                        lhs.checked_sub(rhs).into()
                    }
                }
                Rule::multiply => {
                    if unchecked {
                        let (result, _) = lhs.overflowing_mul(rhs);
                        result.into()
                    } else {
                        lhs.checked_mul(rhs).into()
                    }
                }
                Rule::divide => lhs.checked_div(rhs).into(),
                Rule::power => {
                    if unchecked {
                        let (result, _) = lhs.overflowing_pow(rhs);
                        result.into()
                    } else {
                        lhs.checked_pow(rhs).into()
                    }
                }
                Rule::rightShift => {
                    let shift: usize = match rhs.try_into() {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Error converting to usize: {}", e);
                            return ParseResult::NAN;
                        }
                    };
                    if unchecked {
                        let (result, _) = lhs.overflowing_shr(shift);
                        result.into()
                    } else {
                        lhs.checked_shr(shift).into()
                    }
                }
                Rule::leftShift => {
                    let shift: usize = match rhs.try_into() {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Error converting to usize: {}", e);
                            return ParseResult::NAN;
                        }
                    };
                    if unchecked {
                        let (result, _) = lhs.overflowing_shl(shift);
                        result.into()
                    } else {
                        lhs.checked_shl(shift).into()
                    }
                }
                Rule::modulus => lhs.checked_rem(rhs).into(),
                _ => return ParseResult::NAN,
            }
        },
    )
}

fn utility_fn_str(input: &str, value: &str) -> ParseResult {
    let value = trim_quotes(value);
    match input {
        // evm utils
        "bytes32" => match parse_evm_type(value) {
            Some(value) => value.parse::<B256>().unwrap_or_default().to_string().into(),
            None => ParseResult::NAN,
        },
        "address" | "addr" | "checksum" => {
            let u = value.parse::<U256>().unwrap_or_default();
            u256_to_address(u).to_string().into()
        }
        "keccak256" | "sha3" => keccak256(value).to_string().into(),
        "selector" => keccak256(value.replace(' ', "")).to_string()[..10]
            .to_string()
            .into(),
        "guess_selector" | "fn_from_selector" => "to do".into(),
        // string manipulation
        "len" | "chars" => U256::from(value.len()).into(),
        "lowercase" | "lower" => value.to_lowercase().into(),
        "uppercase" | "upper" => value.to_uppercase().into(),
        "base64_encode" | "b64encode" | "b64_encode" => URL_SAFE.encode(value).into(),
        "base64_decode" | "b64decode" | "b64_decode" => match URL_SAFE.decode(value) {
            Ok(v) => String::from_utf8(v).ok().into(),
            Err(e) => {
                log!("Error decoding base64: {}", e.to_string());
                ParseResult::NAN
            }
        },
        // miscelaneous
        "unix" => U256::from(parse_unix(value)).into(),
        _ => ParseResult::NAN,
    }
}

fn utility_fn_val(input: &str, value: U256) -> ParseResult {
    match input {
        // evm utils
        "bytes32" => B256::from(value).to_string().into(),
        "address" | "addr" | "checksum" => u256_to_address(value).to_string().into(),
        "sqrt" => value.root(2).into(),
        // miscelaneous
        "format_units" | "format_ether" => format_ether(value).into(),
        "unix" => format_unix(value, None),
        _ => ParseResult::NAN,
    }
}

fn utility_fn_args(input: &str, mut pairs: Pairs<Rule>, unchecked: bool) -> ParseResult {
    let value = pairs.next().unwrap();
    let value_str = value.clone().as_str();
    let value_inner = value.into_inner();
    // if value is a quote, value_inner will be empty
    if value_inner.len() == 0 {
        let value_str = trim_quotes(value_str);
        let args = trim_quotes(pairs.next().unwrap().as_str());
        match input {
            "count" => U256::from(count_chars(&value_str, &args)).into(),
            "left_pad" | "lpad" => match args.parse::<u8>() {
                Ok(v) => utils::left_pad(value_str, v.into()).into(),
                Err(e) => {
                    log!("Error parsing left_pad args", e.to_string());
                    ParseResult::NAN
                }
            },
            "right_pad" | "rpad" => match args.parse::<u8>() {
                Ok(v) => utils::right_pad(value_str, v.into()).into(),
                Err(e) => {
                    log!("Error parsing right_pad args", e.to_string());
                    ParseResult::NAN
                }
            },
            "abi_decode" => match abi_process_and_decode_calldata(&value_str, &args) {
                Ok(decoded) => match serde_json::to_value(&decoded) {
                    Ok(json) => json.into(),
                    Err(_) => ParseResult::NAN,
                },
                Err(_) => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        }
    } else {
        match eval(value_inner, unchecked) {
            ParseResult::Value(value) => {
                let args = pairs.next().unwrap().as_str();
                match input {
                    "root" => value.root(args.parse().unwrap_or(2)).into(),
                    "format_units" => format_units(value, args).ok().into(),
                    "unix" => format_unix(value, Some(args.to_string())),
                    _ => ParseResult::NAN,
                }
            }
            _ => ParseResult::NAN,
        }
    }
}

fn parse_encoded_utility_fn(input: &str, name: &str) -> Option<String> {
    if input.starts_with(name) {
        let start = name.len() + 1;
        let end = input.len() - 1;
        if input.len() > start {
            let value = &input[start..end];
            return Some(value.to_owned());
        }
    }
    None
}

fn parse_evm_type(input: String) -> Option<String> {
    if input.starts_with("0x") {
        if input.len() > 1 {
            let value = &input[2..];
            if value.len() % 2 == 0 {
                return Some(value.to_string());
            } else {
                return Some(format!("0{}", value));
            }
        }
        return None;
    }
    None
}

fn parse_unix(input: String) -> i64 {
    let input = input.replace(&['-', '/', ':', 'T'][..], ",");
    let parts: Vec<&str> = input.split(',').collect();
    let mut date_parts = [0 as u32; 6];
    for (i, part) in parts.iter().enumerate() {
        if i < date_parts.len() {
            date_parts[i] = part.parse().unwrap_or(0) as u32;
        }
    }

    let dt = NaiveDateTime::new(
        NaiveDate::from_ymd(date_parts[0] as i32, date_parts[1], date_parts[2]),
        NaiveTime::from_hms(date_parts[3], date_parts[4], date_parts[5]),
    );

    Utc.from_utc_datetime(&dt).timestamp()
}

fn format_unix(u: U256, s_format: Option<String>) -> ParseResult {
    let unix_timestamp: i64 = match u.to_string().parse() {
        Ok(v) => v,
        Err(_) => return ParseResult::NAN,
    };
    let datetime = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    let output = match s_format {
        Some(format) => datetime.format(&format).to_string(),
        None => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    ParseResult::String(trim_quotes(&output))
}
