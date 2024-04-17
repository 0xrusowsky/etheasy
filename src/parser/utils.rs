use alloy_core::primitives::U256;

pub fn stringify(t: Option<U256>) -> String {
    match t {
        None => "-".to_string(),
        Some(u) => u.to_string(),
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
    let trimmed = s.trim_end_matches('0');
    if trimmed.is_empty() { "0" } else { trimmed }.to_string()
}
