#![allow(deprecated)]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use gloo_console::log;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use pest_derive::Parser;

mod convert_chart;
use convert_chart::{convert, UnitType};

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

fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::convert => {
                let mut i = pair.into_inner();
                let value = i.next().unwrap().as_str().parse::<f64>().unwrap();
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
                    f64::NAN
                }
            }
            Rule::function => {
                let mut i = pair.into_inner();
                let name = i.next().unwrap().as_str();
                if name.starts_with("unix") && name.len() > 5 {
                    let end = name.len() - 1;
                    let value = &name[5..end];
                    parse_datetime(value) as f64
                } else {
                    let value = eval(i);
                    math_fn(name, value)
                }
            }
            Rule::pi => std::f64::consts::PI,
            Rule::e => std::f64::consts::E,
            Rule::tau => std::f64::consts::TAU,
            Rule::now => Utc::now().timestamp() as f64,
            Rule::num => pair.as_str().trim().parse::<f64>().unwrap(),
            Rule::hex => {
                let pref_hex = pair.as_str().trim();
                let hex = if pref_hex.starts_with("0x") {
                    &pref_hex[2..]
                } else {
                    &pref_hex[1..]
                };
                i64::from_str_radix(hex, 16).unwrap() as f64
            }
            Rule::bin => {
                let pref_bin = pair.as_str().trim();
                let bin = if pref_bin.starts_with("0b") {
                    &pref_bin[2..]
                } else {
                    &pref_bin[1..]
                };
                i64::from_str_radix(bin, 2).unwrap() as f64
            }
            Rule::expr => eval(pair.into_inner()),
            _ => f64::NAN,
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            Rule::percentOf => percent_of(lhs, rhs),
            Rule::percentOn => percent_on(lhs, rhs),
            Rule::rightShift => (lhs as i64 >> rhs as i64) as f64,
            Rule::leftShift => ((lhs as i64) << rhs as i64) as f64,
            Rule::modulus => (lhs % rhs) as f64,
            _ => f64::NAN,
        },
    )
}

fn percent_on(a: f64, b: f64) -> f64 {
    a / 100_f64 * b + b
}

fn percent_of(a: f64, b: f64) -> f64 {
    a / 100_f64 * b
}

fn math_fn(name: &str, arg: f64) -> f64 {
    match name {
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
        _ => f64::NAN,
    }
}

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

pub fn parse(input: &str) -> f64 {
    let parse_result = Calculator::parse(Rule::calculation, input);
    match parse_result {
        Ok(r) => eval(r),
        Err(_) => f64::NAN,
    }
}

pub fn transform(c: f64) -> String {
    use float_pretty_print::PrettyPrintFloat;
    if c.is_nan() {
        return "-".to_string();
    }
    if c.fract() == 0.0 {
        return c.to_string().trim().to_string();
    }
    return PrettyPrintFloat(c).to_string().trim().to_string();
}
