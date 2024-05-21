#![allow(deprecated)]
mod convert_chart;

#[macro_use]
mod macros;

pub mod types;
pub mod utils;
use crate::components::playground::types::BlockState;
use convert_chart::{convert, UnitType};
use types::{abi::*, result::*};
use utils::{uniswap_v3::*, *};

use alloy_core::primitives::{
    utils::{format_ether, format_units, keccak256},
    B256, U256,
};
use base64::prelude::*;
use chrono::Utc;
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

pub fn parse(input: &str, blocks: &Vec<BlockState>) -> ParseResult {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(r) => eval(r, false, blocks),
        Err(_) => ParseResult::NAN,
    }
}

fn eval(expression: Pairs<Rule>, unchecked: bool, blocks: &Vec<BlockState>) -> ParseResult {
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
            Rule::function => {
                let mut pairs = pair.into_inner();
                let func = pairs.next().unwrap().as_str();
                let unchecked = if func == "unchecked" { true } else { unchecked };
                let args = pairs
                    .map(|pair| match pair.as_rule() {
                        Rule::quote => trim_quotes(pair.as_str()).into(),
                        _ => eval(pair.into_inner(), unchecked, blocks),
                    })
                    .collect::<Vec<ParseResult>>();
                utility_fn_args(func, args)
            }
            Rule::now => U256::from(Utc::now().timestamp()).into(),
            Rule::addr_zero => String::from("0x0000000000000000000000000000000000000000").into(),
            Rule::max_uint => "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .ok()
                .into(),
            Rule::num => {
                let value_str = pair.as_str().trim().to_lowercase();
                if value_str.contains("e") {
                    scientific_to_u256(&value_str).into()
                } else if value_str.starts_with("-") {
                    value_str.into()
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
            Rule::expr => eval(pair.into_inner(), unchecked, blocks),
            Rule::ident => {
                let id = pair.as_str().trim();
                if id == "true" {
                    U256::from(1).into()
                } else if id == "false" {
                    U256::from(0).into()
                } else {
                    match blocks.iter().find(|b| b.get_id() == id) {
                        Some(block) => block.get_result().into(),
                        None => ParseResult::NAN,
                    }
                }
            }
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
                    let shift: usize = unwrap_or_nan!(rhs.try_into());
                    if unchecked {
                        let (result, _) = lhs.overflowing_shr(shift);
                        result.into()
                    } else {
                        lhs.checked_shr(shift).into()
                    }
                }
                Rule::leftShift => {
                    let shift: usize = unwrap_or_nan!(rhs.try_into());
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

const GET_TICK: &[&str] = &[
    "get_tick_from_sqrt_ratio",
    "get_tick_from_sqrt_x96",
    "get_tick_at_sqrt_ratio",
    "tick_from_sqrt_ratio",
    "tick_from_sqrt_x96",
    "tick_at_sqrt_ratio",
    "get_tick_at_sqrt_x96",
    "get_tick",
    "tick",
];

const GET_SQRT_RATIO: &[&str] = &[
    "get_sqrt_ratio_from_tick",
    "get_sqrt_x96_from_tick",
    "get_sqrt_ratio_at_tick",
    "get_sqrt_x96_at_tick",
    "sqrt_ratio_from_tick",
    "sqrt_x96_from_tick",
    "sqrt_ratio_at_tick",
    "sqrt_x96_at_tick",
    "get_sqrt_ratio",
    "get_sqrt_x96",
    "sqrt_ratio",
    "sqrt_x96",
];

const GET_PRICE: &[&str] = &[
    "get_price_from_tick",
    "get_price_at_tick",
    "price_from_tick",
    "price_at_tick",
    "get_price",
];

const GET_QUOTE: &[&str] = &[
    "get_quote_from_tick",
    "get_quote_at_tick",
    "quote_from_tick",
    "quote_at_tick",
    "get_quote",
];

const GET_LIQUIDITY: &[&str] = &[
    "get_liquidity_from_total_amount1",
    "liquidity_from_total_amount1",
    "get_liquidity",
];

const GET_AMOUNT1: &[&str] = &[
    "get_total_amount1_from_liquidity",
    "total_amount1_from_liquidity",
    "get_total_amount1",
];

const GET_AMOUNT0: &[&str] = &[
    "get_amount0_from_liquidity",
    "get_amount0_from_range",
    "amount0_from_liquidity",
    "amount0_from_range",
    "get_amount0",
];

const GET_TOKEN0: &[&str] = &["get_token_0", "get_token0", "token_0", "token0"];
const GET_TOKEN1: &[&str] = &["get_token_1", "get_token1", "token_1", "token1"];

const GET_BOTH_LOWER: &[&str] = &[
    "get_lower_tick_and_sqrt_ratio",
    "get_lower_sqrt_ratio_and_tick",
    "get_lower_sqrt_ratio_both",
    "get_lower_tick_both",
    "get_lower_both",
];

const GET_BOTH_UPPER: &[&str] = &[
    "get_upper_tick_and_sqrt_ratio",
    "get_upper_sqrt_ratio_and_tick",
    "get_upper_sqrt_ratio_both",
    "get_upper_tick_both",
    "get_upper_both",
];

fn utility_fn_args(func: &str, args: Vec<ParseResult>) -> ParseResult {
    if func == "unchecked" && args.len() == 1 {
        return args[0].clone();
    }
    match args.len() {
        1 => match &args[0] {
            ParseResult::String(arg0) => match func {
                // evm utils
                "bytes32" => match parse_evm_type(arg0.to_owned()) {
                    Some(arg0) => arg0.parse::<B256>().unwrap_or_default().to_string().into(),
                    None => ParseResult::NAN,
                },
                "address" | "addr" | "checksum" => {
                    let u = arg0.parse::<U256>().unwrap_or_default();
                    u256_to_address(u).to_string().into()
                }
                "keccak256" | "sha3" => keccak256(arg0).to_string().into(),
                "selector" => keccak256(arg0.replace(' ', "")).to_string()[..10]
                    .to_string()
                    .into(),
                "debug" => {
                    let (prefix, start) = if arg0.starts_with("0x") {
                        match arg0.len() % 64 {
                            2 => (true, 2),
                            10 => (true, 10),
                            _ => (true, arg0.len()),
                        }
                    } else {
                        match arg0.len() % 64 {
                            0 => (false, 0),
                            8 => (false, 8),
                            _ => (false, arg0.len()),
                        }
                    };
                    let mut formatted = if prefix {
                        format!("0x\n{}", &arg0[2..start])
                    } else {
                        arg0[..start].to_string()
                    };
                    for i in (start..arg0.len()).step_by(64) {
                        let end = std::cmp::min(i + 64, arg0.len());
                        formatted = format!("{}\n{}", formatted, &arg0[i..end]);
                    }
                    formatted.into()
                }
                "guess_selector" | "fn_from_selector" => "to do".into(),
                // string manipulation
                "len" | "chars" => U256::from(arg0.len()).into(),
                "lowercase" | "lower" => arg0.to_lowercase().into(),
                "uppercase" | "upper" => arg0.to_uppercase().into(),
                "base64_encode" | "b64encode" | "b64_encode" => BASE64_STANDARD.encode(arg0).into(),
                "base64_decode" | "b64decode" | "b64_decode" => {
                    match BASE64_STANDARD.decode(arg0) {
                        Ok(v) => String::from_utf8(v).ok().into(),
                        Err(e) => {
                            log!("Error decoding base64:", e.to_string());
                            ParseResult::NAN
                        }
                    }
                }
                // uniswap v3 utils
                x if is_command!(x, GET_SQRT_RATIO) => {
                    let tick = unwrap_or_nan!(arg0.parse::<i32>());
                    let sqrt_x96 =
                        unwrap_or_nan!(uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(tick));
                    sqrt_x96.into()
                }
                // miscelaneous
                "unix" => U256::from(parse_unix(arg0.to_owned())).into(),
                _ => ParseResult::NAN,
            },
            ParseResult::Value(arg0) => match func {
                // evm utils
                "bytes32" => B256::from(*arg0).to_string().into(),
                "address" | "addr" | "checksum" => u256_to_address(*arg0).to_string().into(),
                "sqrt" => arg0.root(2).into(),
                // uniswap v3 utils
                x if is_command!(x, GET_TICK) => unwrap_or_nan!(
                    uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(*arg0),
                    "Error getting tick from sqrtX96"
                )
                .to_string()
                .into(),
                x if is_command!(x, GET_SQRT_RATIO) => {
                    let tick = unwrap_or_nan!(arg0.to_string().parse::<i32>());
                    let sqrt_x96 =
                        unwrap_or_nan!(uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(tick));
                    sqrt_x96.into()
                }
                // miscelaneous
                "format_units" | "format_ether" => format_ether(*arg0).into(),
                "unix" => format_unix(*arg0, None),
                _ => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        },
        2 => match (&args[0], &args[1]) {
            (ParseResult::String(arg0), ParseResult::String(arg1)) => match func {
                "count" => U256::from(count_chars(&arg0, &arg1)).into(),
                x if is_command!(x, GET_TOKEN0) => {
                    let token_a = unwrap_or_nan!(arg0.parse::<U256>());
                    let token_b = unwrap_or_nan!(arg1.parse::<U256>());
                    if token_a < token_b {
                        ParseResult::Value(token_a).to_hex_string(false).into()
                    } else {
                        ParseResult::Value(token_b).to_hex_string(false).into()
                    }
                }
                x if is_command!(x, GET_TOKEN1) => {
                    let token_a = unwrap_or_nan!(arg0.parse::<U256>());
                    let token_b = unwrap_or_nan!(arg1.parse::<U256>());
                    if token_a > token_b {
                        ParseResult::Value(token_a).to_hex_string(false).into()
                    } else {
                        ParseResult::Value(token_b).to_hex_string(false).into()
                    }
                }
                "abi_encode" => {
                    let args = split_top_level(trim_parentheses(&arg1));
                    unwrap_or_nan!(abi_encode(&arg0, args, false)).into()
                }
                "abi_encode_with_sig" | "abi_encode_with_selector" => {
                    let args = arg1
                        .split(",")
                        .into_iter()
                        .map(|s| s.trim().to_owned())
                        .collect();
                    unwrap_or_nan!(abi_encode(&arg0, args, true)).into()
                }
                "abi_decode" => match abi_process_and_decode_calldata(&arg0, &arg1) {
                    (Some(selector), Ok(decoded)) => {
                        match serde_json::to_value(&decoded) {
                            Ok(mut json) => {
                                // Convert serde_json::Value into Vec<serde_json::Value>
                                if let Some(array) = json.as_array_mut() {
                                    array.insert(0, serde_json::to_value(&selector).unwrap());
                                }
                                // Convert Vec<serde_json::Value> back into serde_json::Value
                                let json =
                                    serde_json::Value::Array(json.as_array().unwrap().clone());
                                json.into()
                            }
                            Err(_) => ParseResult::NAN,
                        }
                    }
                    (None, Ok(decoded)) => match serde_json::to_value(&decoded) {
                        Ok(json) => json.into(),
                        Err(_) => ParseResult::NAN,
                    },
                    (_, Err(_)) => ParseResult::NAN,
                },
                _ => ParseResult::NAN,
            },
            (ParseResult::String(arg0), ParseResult::Value(arg1)) => match func {
                "left_pad" | "lpad" => {
                    let units: usize = unwrap_or_nan!(arg1.to_string().parse::<u8>()).into();
                    utils::left_pad(arg0, units).into()
                }
                "right_pad" | "rpad" => {
                    let units: usize = unwrap_or_nan!(arg1.to_string().parse::<u8>()).into();
                    utils::right_pad(arg0, units).into()
                }
                _ => ParseResult::NAN,
            },
            (ParseResult::Value(arg0), ParseResult::Value(arg1)) => match func {
                "root" => arg0.root(arg1.to_string().parse().unwrap_or(2)).into(),
                "format_units" => format_units(*arg0, arg1.to_string()).ok().into(),
                x if is_command!(x, GET_TOKEN0) => {
                    if arg0 < arg1 {
                        ParseResult::Value(*arg0).to_hex_string(false).into()
                    } else {
                        ParseResult::Value(*arg1).to_hex_string(false).into()
                    }
                }
                x if is_command!(x, GET_TOKEN1) => {
                    if arg0 > arg1 {
                        ParseResult::Value(*arg0).to_hex_string(false).into()
                    } else {
                        ParseResult::Value(*arg1).to_hex_string(false).into()
                    }
                }
                _ => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        },
        3 => match (&args[0], &args[1], &args[2]) {
            (ParseResult::Value(arg0), ParseResult::Value(arg1), ParseResult::Value(arg2)) => {
                match func {
                    "unix" => build_unix(vec![arg0, arg1, arg2]).into(),
                    "get_lower_tick" => match get_lower_tick(*arg0, *arg1, *arg2) {
                        Some(tick) => tick.to_string().into(),
                        None => ParseResult::NAN,
                    },
                    "get_upper_tick" => match get_upper_tick(*arg0, *arg1, *arg2) {
                        Some(tick) => tick.to_string().into(),
                        None => ParseResult::NAN,
                    },
                    "get_lower_sqrt_ratio" => get_lower_sqrt_price(*arg0, *arg1, *arg2).into(),
                    "get_upper_sqrt_ratio" => get_lower_sqrt_price(*arg0, *arg1, *arg2).into(),
                    x if is_command!(x, GET_BOTH_LOWER) => {
                        get_both_lower(*arg0, *arg1, *arg2).into()
                    }
                    x if is_command!(x, GET_BOTH_UPPER) => {
                        get_both_upper(*arg0, *arg1, *arg2).into()
                    }
                    _ => ParseResult::NAN,
                }
            }
            _ => ParseResult::NAN,
        },
        4 => match (&args[0], &args[1], &args[2], &args[3]) {
            (
                ParseResult::Value(arg0),
                ParseResult::Value(arg1),
                ParseResult::Value(arg2),
                ParseResult::Value(arg3),
            ) => match func {
                x if is_command!(x, GET_PRICE) => get_price!(*arg0, *arg1, *arg2, *arg3, true),
                x if is_command!(x, GET_QUOTE) => get_price!(*arg0, *arg1, *arg2, *arg3, false),
                x if is_command!(x, GET_LIQUIDITY) => {
                    get_v3_liquidity(*arg0, *arg1, *arg2, *arg3).into()
                }
                x if is_command!(x, GET_AMOUNT0) => {
                    get_amount0_from_v3_range(*arg0, *arg1, *arg2, *arg3).into()
                }
                x if is_command!(x, GET_AMOUNT1) => {
                    get_amount1_from_v3_range(*arg0, *arg1, *arg2, *arg3).into()
                }
                "unix" => build_unix(vec![arg0, arg1, arg2, arg3]).into(),
                _ => ParseResult::NAN,
            },
            (
                ParseResult::String(arg0), // negative ticks are parsed as strings
                ParseResult::Value(arg1),
                ParseResult::Value(arg2),
                ParseResult::Value(arg3),
            ) => match func {
                x if is_command!(x, GET_PRICE) => get_price!(*arg0, *arg1, *arg2, *arg3, true),
                x if is_command!(x, GET_QUOTE) => get_price!(*arg0, *arg1, *arg2, *arg3, false),
                _ => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        },
        5 => match (&args[0], &args[1], &args[2], &args[3], &args[4]) {
            (
                ParseResult::Value(arg0),
                ParseResult::Value(arg1),
                ParseResult::Value(arg2),
                ParseResult::Value(arg3),
                ParseResult::Value(arg4),
            ) => match func {
                "unix" => build_unix(vec![arg0, arg1, arg2, arg3, arg4]).into(),
                _ => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        },
        6 => match (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5]) {
            (
                ParseResult::Value(arg0),
                ParseResult::Value(arg1),
                ParseResult::Value(arg2),
                ParseResult::Value(arg3),
                ParseResult::Value(arg4),
                ParseResult::Value(arg5),
            ) => match func {
                "unix" => build_unix(vec![arg0, arg1, arg2, arg3, arg4, arg5]).into(),
                _ => ParseResult::NAN,
            },
            _ => ParseResult::NAN,
        },
        _ => ParseResult::NAN,
    }
}
