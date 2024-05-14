#[macro_export]
macro_rules! unwrap_or_nan {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                gloo_console::log!("Error:", e.to_string());
                return ParseResult::NAN;
            }
        }
    };
    ($expr:expr, $error:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                gloo_console::log!(format!("{}: {}", $error, e.to_string()));
                return ParseResult::NAN;
            }
        }
    };
}

#[macro_export]
macro_rules! is_command {
    ($x:expr, $list:expr) => {
        $list.contains(&$x)
    };
}

#[macro_export]
macro_rules! get_price {
    ($tick_u256:expr, $in_token1:expr, $decimals0:expr, $decimals1:expr, $format_output:expr) => {{
        let tick = unwrap_or_nan!($tick_u256.to_string().parse::<i32>());
        let in_token1 = match $in_token1 {
            ZERO => false,
            ONE => true,
            _ => return ParseResult::NAN,
        };
        let quote =
            utils::uniswap_v3::get_v3_quote_from_tick(tick, $decimals0, $decimals1, in_token1);
        if $format_output {
            let units = if in_token1 { $decimals1 } else { $decimals0 };
            let price = unwrap_or_nan!(format_units(quote, units.to_string()));
            format!(
                "1 {} : {} {}",
                if in_token1 { "token0" } else { "token1" },
                price,
                if in_token1 { "token1" } else { "token0" },
            )
            .into()
        } else {
            quote.into()
        }
    }};
}
