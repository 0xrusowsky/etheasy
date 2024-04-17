use alloy_core::primitives::{B256, U256};
use gloo_console::*;

pub fn stringify(u: Option<U256>, full: bool) -> (String, String) {
    match u {
        None => ("-".to_string(), "-".to_string()),
        Some(u) => {
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
                log!("hex_str length: {}", hex_str.len());
                // For the `full` true case, check if the hex string is exactly 66 characters long
                let formatted_hex = format!(
                    "0x\n{}\n{}",
                    hex_str[2..34].to_string(),
                    hex_str[34..].to_string()
                );
                (format!("-\n-\n{}", dec), formatted_hex)
            }
        }
    }
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
