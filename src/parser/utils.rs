use alloy_core::primitives::{B256, U256};
use gloo_console::*;

use super::types::ParseResult;

pub fn stringify(u: ParseResult, full: bool) -> (String, String) {
    match u {
        ParseResult::NAN => ("-".to_string(), "-".to_string()),
        ParseResult::Value(u) => {
            let dec = u.to_string();
            let hex: B256 = u.into();
            let hex_str = hex.to_string();

            // When `full` is `false`, trim the leading zeros from the hex representation
            if !full {
                let hex_formatted = format!("0x{}", hex_str[2..].trim_start_matches("0"));
                if hex_formatted == "0x" {
                    (dec, "0x0".to_string())
                } else {
                    (dec, hex_formatted)
                }
            } else {
                ("-".to_string(), hex_str)
            }
        }
        ParseResult::String(mut s) => {
            if !s.starts_with("0x") {
                s = format!("'{}'", s);
            }
            ("-".to_string(), s)
        }
    }
}

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

pub fn lead_zeros(s: &str) -> usize {
    s.chars().take_while(|&c| c == '0').count()
}

pub fn remove_trailing_zeros(s: &str) -> String {
    let trimmed = s.trim_end_matches("0");
    if trimmed.is_empty() { "0" } else { trimmed }.to_string()
}
