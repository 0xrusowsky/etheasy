use alloy_core::primitives::U256;
use uniswap_v3_math::full_math::mul_div_rounding_up;

const TWO: U256 = U256::from_limbs([2, 0, 0, 0]);
const Q96: U256 = uniswap_v3_math::sqrt_price_math::Q96;
const MIN_TICK: i32 = uniswap_v3_math::tick_math::MIN_TICK;
const MAX_TICK: i32 = uniswap_v3_math::tick_math::MAX_TICK;
const MIN_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MIN_SQRT_RATIO;
const MAX_SQRT_RATIO: U256 = uniswap_v3_math::tick_math::MAX_SQRT_RATIO;

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
    };

    Some(quote)
}

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
        Some(U256::from(0))
    } else if sqrt_price > sqrt_pb {
        mul_div_rounding_up(liquidity, sqrt_pb - sqrt_pa, Q96).ok()
    } else {
        mul_div_rounding_up(liquidity, sqrt_price - sqrt_pa, Q96).ok()
    }
}
