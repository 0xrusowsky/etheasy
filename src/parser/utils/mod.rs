pub mod uniswap_v3;
use super::types::result::ParseResult;

use alloy_core::primitives::{Address, B256, U256};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

pub const ZERO: U256 = U256::from_limbs([0, 0, 0, 0]);
pub const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);

pub fn trim_quotes(input: &str) -> String {
    let mut chars = input.chars();
    // Check if the first character is a quote
    if let Some(first) = chars.next() {
        if first == '"' || first == '\'' {
            // Check if the last character is also a quote of the same type
            // Must collect into Vec to access the last element as `chars` is an iterator
            let mut chars: Vec<char> = chars.collect();
            if chars.pop() == Some(first) {
                // If both conditions are true, return the string without the first and last character
                return chars.into_iter().collect();
            }
            // If only the first character was a quote, rebuild the string with the remaining characters
            chars.insert(0, first);
            return chars.into_iter().collect();
        }
    }
    // Return the original input if no modifications were made
    input.to_string()
}

pub fn u256_to_address(u: U256) -> Address {
    let a = B256::from(u).to_string();
    format!("0x{}", &a[26..])
        .parse::<Address>()
        .unwrap_or_default()
}

pub fn scientific_to_u256(s: &str) -> Option<U256> {
    let mut split_iter = s.split("e");
    let mut float_iter = split_iter.next().unwrap_or("0").split(".");
    let mut exp = split_iter.next().unwrap_or("0").parse::<u64>().unwrap();

    // process integer and fractional parts separetely
    let base_str = float_iter.next().unwrap_or("0");
    let frac_str = remove_trailing_zeros(float_iter.next().unwrap_or("0"));
    let frac_units = frac_str.chars().count() as u64;
    // merge integer and fractional parts (risk of precision loss here)
    let uint = format!("{base_str}{frac_str}").parse::<U256>().unwrap();
    if exp >= frac_units {
        exp -= frac_units;
        uint.checked_mul(U256::from(10).checked_pow(U256::from(exp))?)
    } else {
        exp = frac_units - exp;
        uint.checked_div(U256::from(10).checked_pow(U256::from(exp))?)
    }
}

pub fn left_pad(s: &String, width: usize) -> String {
    format!("{:0>width$}", s, width = width)
}

pub fn right_pad(s: &String, width: usize) -> String {
    format!("{:0<width$}", s, width = width)
}

pub fn remove_trailing_zeros(s: &str) -> String {
    let trimmed = s.trim_end_matches("0");
    if trimmed.is_empty() { "0" } else { trimmed }.to_string()
}

pub fn count_chars(s: &str, c: &str) -> usize {
    s.len() - s.replace(c, "").len()
}

pub fn trim_parentheses(input: &str) -> &str {
    let mut chars = input.chars();
    let first = chars.next();
    let last = chars.last();

    match (first, last) {
        (Some('('), Some(')')) => &input[1..input.len() - 1],
        (Some('('), _) => &input[1..],
        (_, Some(')')) => &input[..input.len() - 1],
        _ => input,
    }
}

pub fn split_top_level(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;
    let mut parenthesis_depth = 0;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ',' if bracket_depth == 0 && parenthesis_depth == 0 => {
                // If we're not inside any brackets or parentheses, split here
                result.push(current.trim().to_string());
                current = String::new();
            }
            '[' => {
                bracket_depth += 1;
                current.push(c);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(c);
            }
            '(' => {
                parenthesis_depth += 1;
                current.push(c);
            }
            ')' => {
                parenthesis_depth -= 1;
                current.push(c);
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        result.push(current.trim().to_string());
    }

    result
}

pub fn parse_evm_type(input: String) -> Option<String> {
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

pub fn parse_unix(input: String) -> i64 {
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

pub fn build_unix(parts: Vec<&U256>) -> i64 {
    let mut date_parts = [0 as u32; 6];
    for (i, part) in parts.iter().enumerate() {
        if i < date_parts.len() {
            date_parts[i] = part.to_string().parse().unwrap_or(0) as u32;
        }
    }

    let dt = NaiveDateTime::new(
        NaiveDate::from_ymd(date_parts[0] as i32, date_parts[1], date_parts[2]),
        NaiveTime::from_hms(date_parts[3], date_parts[4], date_parts[5]),
    );

    Utc.from_utc_datetime(&dt).timestamp()
}

pub fn format_unix(u: U256, s_format: Option<String>) -> ParseResult {
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
