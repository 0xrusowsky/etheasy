#![allow(deprecated)]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use gloo_console::log;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use pest_derive::Parser;

mod convert_chart;
use convert_chart::{convert, UnitType};

mod types;
pub use types::U256;

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
            Operator::new(percentOf, Left) | Operator::new(percentOn, Left),
            Operator::new(rightShift, Right) | Operator::new(leftShift, Right),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> Option<U256> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::convert => {
                let mut i = pair.into_inner();
                let value = i.next().unwrap().as_str().parse::<U256>().unwrap();
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
                    convert(value, from, to)
                } else {
                    None
                }
            }
            Rule::function => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                if name.starts_with("unix") && name.len() > 5 {
                    let end = name.len() - 1;
                    let value = &name[5..end];
                    Some(U256::from(parse_datetime(value)))
                } else {
                    let value = eval(i)?;
                    None
                    // math_fn(name, value)
                }
            }
            Rule::now => Some(U256::from(Utc::now().timestamp())),
            Rule::num => {
                let value_str = pair.as_str().trim();
                if value_str.contains("e") {
                    scientific_to_u256(value_str)
                } else {
                    value_str.parse::<U256>().ok()
                }
            }
            Rule::hex => {
                let pref_hex = pair.as_str().trim();
                let hex = if pref_hex.starts_with("0x") {
                    &pref_hex[2..]
                } else {
                    &pref_hex[1..]
                };
                U256::from_str_radix(hex, 16).ok()
            }
            Rule::bin => {
                let pref_bin = pair.as_str().trim();
                let bin = if pref_bin.starts_with("0b") {
                    &pref_bin[2..]
                } else {
                    &pref_bin[1..]
                };
                U256::from_str_radix(bin, 2).ok()
            }
            Rule::expr => eval(pair.into_inner()),
            _ => None,
        },
        |lhs: Option<U256>, op: Pair<Rule>, rhs: Option<U256>| {
            let lhs = lhs?;
            let rhs = rhs?;
            match op.as_rule() {
                Rule::add => lhs.checked_add(rhs),
                Rule::subtract => lhs.checked_sub(rhs),
                Rule::multiply => lhs.checked_mul(rhs),
                Rule::divide => lhs.checked_div(rhs),
                Rule::power => lhs.checked_pow(rhs),
                Rule::percentOf => percent_of(lhs, rhs),
                Rule::percentOn => percent_on(lhs, rhs),
                Rule::rightShift => {
                    let shift: usize = match rhs.try_into() {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Error converting to usize: {}", e);
                            return None;
                        }
                    };
                    lhs.checked_shr(shift)
                }
                Rule::leftShift => {
                    let shift: usize = match rhs.try_into() {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Error converting to usize: {}", e);
                            return None;
                        }
                    };
                    lhs.checked_shl(shift)
                }
                Rule::modulus => lhs.checked_rem(rhs),
                _ => return None,
            }
        },
    )
}

fn percent_on(a: U256, b: U256) -> Option<U256> {
    a.checked_mul(b)?
        .checked_div(U256::from(100))?
        .checked_add(b)
}

fn percent_of(a: U256, b: U256) -> Option<U256> {
    a.checked_mul(b)?.checked_div(U256::from(100))
}

/*
fn math_fn(name: &str, arg: f64) -> Option<U256> {
    let result = match name {
        "sin" => arg.to_radians().sin(),
        "cos" => arg.to_radians().cos(),
        "tan" => arg.to_radians().tan(),
        "asin" => arg.asin(),
        "acos" => arg.cos(),
        "atan" => arg.atan(),
        "sinh" => arg.sinh(),
        "cosh" => arg.cosh(),
        "tanh" => arg.tanh(),
        "asinh" => arg.asinh(),
        "acosh" => arg.acosh(),
        "atanh" => arg.atanh(),
        "log" => arg.log10(),
        "sqrt" => arg.sqrt(),
        "cbrt" => arg.cbrt(),
        "round" => arg.round(),
        "ceil" => arg.ceil(),
        "floor" => arg.floor(),
        _ => return None,
    };
    result
}
*/

fn parse_datetime(input: &str) -> i64 {
    let std_input = input.replace(&['-', '/', ':', 'T', '+'][..], ",");
    let parts: Vec<&str> = std_input.split(',').collect();
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

pub fn parse(input: &str) -> Option<U256> {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(r) => eval(r),
        Err(_) => None,
    }
}

pub fn transform(t: Option<U256>) -> String {
    match t {
        None => "-".to_string(),
        Some(u) => u.to_string(),
    }
}

fn scientific_to_u256(s: &str) -> Option<U256> {
    let mut split_iter = s.split("e");
    let mut base_iter = split_iter.next().unwrap_or("0").split(".");
    let exp = split_iter.next().unwrap_or("0").parse::<u64>().unwrap();

    // process integer part
    let base_int = base_iter.next().unwrap_or("0");
    let base_units = base_int.chars().count() as u64;
    let base_int = base_int.parse::<U256>().unwrap();

    // process fractional part
    let base_frac_str = remove_trailing_zeros(base_iter.next().unwrap_or("0"));
    let base_frac = base_frac_str.parse::<U256>().unwrap();
    if base_units + lead_zeros(&base_frac_str) as u64 <= exp {
        let frac_units = exp - base_units - lead_zeros(&base_frac_str) as u64;
        let exp_base = U256::from(10)
            .checked_pow(U256::from(exp))
            .unwrap_or(U256::from(0));
        let exp_frac = U256::from(10)
            .checked_pow(U256::from(frac_units))
            .unwrap_or(U256::from(0));
        base_int
            .checked_mul(exp_base)?
            .checked_add(base_frac.checked_mul(exp_frac)?)
    } else {
        Some(U256::from(0))
    }
}

fn lead_zeros(s: &str) -> usize {
    s.chars().take_while(|&c| c == '0').count()
}

fn remove_trailing_zeros(s: &str) -> String {
    let trimmed = s.trim_end_matches('0');
    if trimmed.is_empty() { "0" } else { trimmed }.to_string()
}
