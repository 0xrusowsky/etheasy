use alloy_core::primitives::U256;
use uniswap_v3_math::{
    full_math::{mul_div, mul_div_rounding_up},
    tick_math::get_tick_at_sqrt_ratio,
};

const ZERO: U256 = U256::from_limbs([0, 0, 0, 0]);
const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);
const TWO: U256 = U256::from_limbs([2, 0, 0, 0]);
const Q96: U256 = uniswap_v3_math::sqrt_price_math::Q96;
const DENOM_9: U256 = U256::from_limbs([1000000000, 0, 0, 0]);
const DENOM_18: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);
const MIN_TICK: i32 = uniswap_v3_math::tick_math::MIN_TICK;
const MAX_TICK: i32 = uniswap_v3_math::tick_math::MAX_TICK;
const MIN_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MIN_SQRT_RATIO;
const MAX_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MAX_SQRT_RATIO;

fn _get_tick_at_sqrt_ratio(sqrt_price: U256) -> Option<i32> {
    match sqrt_price {
        MIN_SQRT_RATIO => Some(MIN_TICK),
        MAX_SQRT_RATIO => Some(MAX_TICK),
        _ => get_tick_at_sqrt_ratio(sqrt_price).ok(),
    }
}

pub fn get_v3_quote_from_tick(
    tick: i32,
    decimals0: U256,
    decimals1: U256,
    in_token1: bool,
) -> Option<U256> {
    if tick < MIN_TICK || tick > MAX_TICK {
        return None;
    }

    let sqrt_ratio_x96 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(tick)
        .unwrap()
        .to_string()
        .parse::<U256>()
        .unwrap();

    let quote = if sqrt_ratio_x96 <= i128::MAX.to_string().parse::<U256>().unwrap() {
        let ratio_x192 = sqrt_ratio_x96 * sqrt_ratio_x96;
        if in_token1 {
            mul_div(
                U256::from(ratio_x192),
                U256::from(10).pow(decimals0),
                U256::from(1) << U256::from(192),
            )
        } else {
            mul_div(
                U256::from(1) << U256::from(192),
                U256::from(10).pow(decimals1),
                U256::from(ratio_x192),
            )
        }
    } else {
        let ratio_x128 = sqrt_ratio_x96 * sqrt_ratio_x96 / (U256::from(1) << U256::from(64));
        if in_token1 {
            mul_div(
                U256::from(ratio_x128),
                U256::from(10).pow(decimals0),
                U256::from(1) << U256::from(128),
            )
        } else {
            mul_div(
                U256::from(1) << U256::from(128),
                U256::from(10).pow(decimals1),
                U256::from(ratio_x128),
            )
        }
    };

    quote.ok()
}

pub fn get_v3_liquidity(
    total_amount1: U256,
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
        Some(Q96 * total_amount1 * sqrt_pb * sqrt_pa / sqrt_price.pow(TWO) / (sqrt_pb - sqrt_pa))
    } else if sqrt_price > sqrt_pb {
        Some(Q96 * total_amount1 / (sqrt_pb - sqrt_pa))
    } else {
        Some(Q96 * total_amount1 / (sqrt_price * TWO - (sqrt_price.pow(TWO) / sqrt_pb) - sqrt_pa))
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
        let t1 = mul_div_rounding_up(liquidity, Q96, sqrt_pa);
        let t2 = mul_div_rounding_up(liquidity, Q96, sqrt_pb);
        if t1.is_ok() && t2.is_ok() {
            let t1 = t1.unwrap();
            let t2 = t2.unwrap();
            if t1 >= t2 {
                return Some(t1 - t2);
            }
        }
        None
    } else if sqrt_price > sqrt_pb {
        Some(U256::from(0))
    } else {
        let t1 = mul_div_rounding_up(liquidity, Q96, sqrt_price);
        let t2 = mul_div_rounding_up(liquidity, Q96, sqrt_pb);
        if t1.is_ok() && t2.is_ok() {
            let t1 = t1.unwrap();
            let t2 = t2.unwrap();
            if t1 >= t2 {
                return Some(t1 - t2);
            }
        }
        None
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
        Some(ZERO)
    } else if sqrt_price > sqrt_pb {
        mul_div_rounding_up(liquidity, sqrt_pb - sqrt_pa, Q96).ok()
    } else {
        mul_div_rounding_up(liquidity, sqrt_price - sqrt_pa, Q96).ok()
    }
}

pub fn get_lower_sqrt_price(liquidity: U256, use_amount1: U256, sqrt_price: U256) -> Option<U256> {
    if liquidity == ZERO || sqrt_price > MAX_SQRT_RATIO || sqrt_price < MIN_SQRT_RATIO {
        return None;
    };

    let impact = use_amount1 * Q96 / liquidity;
    if impact >= sqrt_price {
        Some(MIN_SQRT_RATIO)
    } else {
        Some(sqrt_price - impact)
    }
}

pub fn get_lower_tick(liquidity: U256, use_amount1: U256, sqrt_price: U256) -> Option<i32> {
    match get_lower_sqrt_price(liquidity, use_amount1, sqrt_price) {
        None => None,
        Some(sqrt_pa) => _get_tick_at_sqrt_ratio(sqrt_pa),
    }
}

pub fn get_both_lower(liquidity: U256, use_amount1: U256, sqrt_price: U256) -> Option<String> {
    match get_lower_sqrt_price(liquidity, use_amount1, sqrt_price) {
        None => None,
        Some(sqrt_pa) => Some(format!(
            "sqrtPa: {}\n tick: {}",
            &sqrt_pa,
            match _get_tick_at_sqrt_ratio(sqrt_pa) {
                Some(tick) => tick,
                None => return None,
            }
        )),
    }
}

pub fn get_upper_sqrt_price(liquidity: U256, use_amount0: U256, sqrt_price: U256) -> Option<U256> {
    if liquidity == ZERO || sqrt_price > MAX_SQRT_RATIO || sqrt_price < MIN_SQRT_RATIO {
        return None;
    };

    let impact = use_amount0 * sqrt_price;

    if impact >= Q96 * liquidity {
        gloo_console::log!("MAX_SQRT_RATIO");
        Some(MAX_SQRT_RATIO)
    } else {
        gloo_console::log!("div:", (Q96 * liquidity - impact).to_string());
        gloo_console::log!("num:", (Q96 * liquidity).to_string());
        Some(sqrt_price * liquidity * Q96 / (Q96 * liquidity - impact))
    }
}

pub fn get_upper_tick(liquidity: U256, use_amount0: U256, sqrt_price: U256) -> Option<i32> {
    match get_upper_sqrt_price(liquidity, use_amount0, sqrt_price) {
        None => None,
        Some(sqrt_pb) => {
            gloo_console::log!("sqrt_pb:", sqrt_pb.to_string());
            _get_tick_at_sqrt_ratio(sqrt_pb)
        }
    }
}

pub fn get_both_upper(liquidity: U256, use_amount0: U256, sqrt_price: U256) -> Option<String> {
    match get_upper_sqrt_price(liquidity, use_amount0, sqrt_price) {
        None => None,
        Some(sqrt_pb) => Some(format!(
            "sqrtPb: {}\n tick: {}",
            &sqrt_pb,
            match _get_tick_at_sqrt_ratio(sqrt_pb) {
                Some(tick) => tick,
                None => return None,
            }
        )),
    }
}

fn price_input_to_u256(price: PriceInput) -> (U256, bool) {
    match price {
        PriceInput::S(s) => {
            let mut s = s.split('.');
            let int = s.next().unwrap().parse::<U256>().unwrap();
            let (frac_len, frac) = match s.next() {
                Some(f) => (f.len(), f.parse::<U256>().unwrap()),
                None => (0, ZERO),
            };
            (
                int * DENOM_18 + frac * U256::from(10).pow(U256::from(18 - frac_len)),
                true,
            )
        }
        PriceInput::U(u) => {
            if u < "115792089237316195423570985008687907853269984665640564039457"
                .parse::<U256>()
                .unwrap()
            {
                (u * DENOM_18, true)
            } else {
                (u, false)
            }
        }
    }
}
pub fn price_to_sqrt_ratio(price: PriceInput) -> Option<U256> {
    // Calculate the square root with the max possible precision
    let (price, scaled) = price_input_to_u256(price);

    // Handle integer and fractional parts separately
    let sqrt = price.root(2);
    let result = mul_div(sqrt, Q96, if scaled { DENOM_9 } else { ONE }).unwrap();

    if result > MAX_SQRT_RATIO || result < MIN_SQRT_RATIO {
        None
    } else {
        Some(result)
    }
}

pub fn get_pool_tick(tick: i32, spacing: u32) -> Option<i32> {
    let spacing = spacing as i32;
    if spacing == 0 || tick < MIN_TICK || tick > MAX_TICK {
        return None;
    }
    Some(tick / spacing * spacing)
}

pub enum PriceInput {
    S(String),
    U(U256),
}
