use std::i32;

use alloy_core::primitives::{Address, B256, U256};

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

pub fn left_pad(s: String, width: usize) -> String {
    format!("{:0>width$}", s, width = width)
}

pub fn right_pad(s: String, width: usize) -> String {
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

// uniswap v3 utils
pub fn get_v3_quote_from_tick(
    tick: i32,
    decimals0: U256,
    decimals1: U256,
    in_token1: bool,
) -> U256 {
    let sqrt_ratio_x96 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(tick)
        .unwrap()
        .to_string()
        .parse::<U256>()
        .unwrap();

    if sqrt_ratio_x96 <= i128::MAX.to_string().parse::<U256>().unwrap() {
        let ratio_x192 = sqrt_ratio_x96.pow(U256::from(2));
        if in_token1 {
            U256::from(ratio_x192) * U256::from(10).pow(decimals0)
                / (U256::from(1) << U256::from(192))
        } else {
            (U256::from(1) << U256::from(192)) * U256::from(10).pow(decimals1)
                / U256::from(ratio_x192)
        }
    } else {
        let ratio_x128 = sqrt_ratio_x96 * sqrt_ratio_x96 / (U256::from(1) << U256::from(64));
        if in_token1 {
            U256::from(ratio_x128) * U256::from(10).pow(decimals0)
                / (U256::from(1) << U256::from(128))
        } else {
            (U256::from(1) << U256::from(128)) * U256::from(10).pow(decimals1)
                / U256::from(ratio_x128)
        }
    }
}

const TWO: U256 = uniswap_v3_math::full_math::TWO;
const Q96: U256 = uniswap_v3_math::sqrt_price_math::Q96;
const MIN_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MIN_SQRT_RATIO;
const MAX_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MAX_SQRT_RATIO;

pub fn get_v3_liquidity(
    amount1: U256,
    sqrt_price: U256,
    sqrt_pa: U256,
    sqrt_pb: U256,
) -> Option<U256> {
    if sqrt_price < MIN_SQRT_RATIO
        || sqrt_price > MAX_SQRT_RATIO
        || sqrt_pa < MIN_SQRT_RATIO
        || sqrt_pa > MAX_SQRT_RATIO
        || sqrt_pb < MIN_SQRT_RATIO
        || sqrt_pb > MAX_SQRT_RATIO
    {
        return None;
    }

    if sqrt_price < sqrt_pa {
        Some(Q96 * amount1 * sqrt_pb * sqrt_pa / sqrt_price.pow(TWO) / (sqrt_pb - sqrt_pa))
    } else if sqrt_price > sqrt_pb {
        Some(Q96 * amount1 / (sqrt_pb - sqrt_pa))
    } else {
        Some(Q96 * amount1 / (sqrt_price * TWO - (sqrt_price.pow(TWO) / sqrt_pb) - sqrt_pa))
    }
}

pub fn get_amount0_from_v3_range(
    liquidity: U256,
    sqrt_price: U256,
    sqrt_pa: U256,
    sqrt_pb: U256,
) -> Option<U256> {
    if sqrt_price < MIN_SQRT_RATIO
        || sqrt_price > MAX_SQRT_RATIO
        || sqrt_pa < MIN_SQRT_RATIO
        || sqrt_pa > MAX_SQRT_RATIO
        || sqrt_pb < MIN_SQRT_RATIO
        || sqrt_pb > MAX_SQRT_RATIO
    {
        return None;
    }

    if sqrt_price < sqrt_pa {
        Some(Q96 * liquidity / sqrt_pa - Q96 * liquidity / sqrt_pb)
    } else if sqrt_price > sqrt_pb {
        Some(U256::from(0))
    } else {
        Some(Q96 * liquidity * sqrt_price - Q96 * liquidity / sqrt_pb)
    }
}

pub fn get_amount1_from_v3_range(
    liquidity: U256,
    sqrt_price: U256,
    sqrt_pa: U256,
    sqrt_pb: U256,
) -> Option<U256> {
    if sqrt_price < MIN_SQRT_RATIO
        || sqrt_price > MAX_SQRT_RATIO
        || sqrt_pa < MIN_SQRT_RATIO
        || sqrt_pa > MAX_SQRT_RATIO
        || sqrt_pb < MIN_SQRT_RATIO
        || sqrt_pb > MAX_SQRT_RATIO
    {
        return None;
    }

    if sqrt_price < sqrt_pa {
        Some(U256::from(0))
    } else if sqrt_price > sqrt_pb {
        Some(liquidity * (sqrt_pb - sqrt_pa) / Q96)
    } else {
        Some(liquidity * (sqrt_price - sqrt_pa) / Q96)
    }
}
