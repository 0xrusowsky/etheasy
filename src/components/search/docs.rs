#[derive(Debug, Clone, PartialEq)]
pub struct SearchItemData {
    pub id: usize,
    pub command: &'static str,
    pub c_type: CommandType,
    pub alias: Option<&'static str>,
    pub params: Option<&'static str>,
    pub example: Option<&'static str>,
    pub desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Input,
    Function,
    Constant,
    Operation,
    Conversion,
}

impl CommandType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CommandType::Input => "Input type",
            CommandType::Function => "Function",
            CommandType::Constant => "Constant",
            CommandType::Operation => "Operation",
            CommandType::Conversion => "Conversion",
        }
    }
}

pub static SEARCH_ITEMS: &[SearchItemData; 49] = &[
    // START: INPUT COMMANDS
    SearchItemData {
        id: 0,
        command: "variable",
        c_type: CommandType::Input,
        alias: None,
        params: None,
        example: None,
        desc: "Block results are stored in the app state, and can be referenced in other blocks.\nBy default names follow `block_x` notation, but you can be renamed by modifying the block label.\nVariables don't need to be wrapped in quotes, and will evaluate to their corresponding value.",
    },
    SearchItemData {
        id: 1,
        command: "uint256",
        c_type: CommandType::Input,
        alias: None,
        params: None,
        example: Some("0b1010   // bin (10)\n0x1234   // hex (460)\n987650   // hex (987650)\n1.25e6   // scientific notation (1250000)"),
        desc: "EVM word that consists of up to 32 bytes or 256 bits representing an unsigned integer.\nAccepts decimal, hex, or binary inputs. Also supports scientific notation for decimals.\nDue to its integer nature, it doesn't support floating point numbers.\nDue to its unsigned nature, it doesn't support negative numbers.",
    },
    SearchItemData {
        id: 2,
        command: "string",
        c_type: CommandType::Input,
        alias: None,
        params: None,
        example: Some("\"Hello, World!\"   // double quotes ('Hello, World!')\n'Foo Bar'         // single quotes ('Foo Bar')"),
        desc: "Strings must be wrapped around quotation marks. Both single and double quotes are accepted.",
    },
    // END: INPUT COMMANDS
    SearchItemData {
        id: 3,
        command: "unchecked",
        c_type: CommandType::Function,
        alias: None,
        params: None,
        example: Some("unchecked(0 - 1)                     // unchecked operation (max_uint)\nunchecked(format_units(2**256, 4))   // unchecked composition ('0.0000')"),
        desc: "Enables unchecked math for any calculation performed inside its brackets.",
    },
    // START: OPERATION COMMANDS
    SearchItemData {
        id: 4,
        command: "+",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("0x11 + 3                  // addition (20)\nmax_uint + 3              // overflowing addition (-) \nunchecked(max_uint + 3)   // unchecked overflowing addition (2)"),
        desc: "Computes the non-overflowing addition of two values.",
    },
    SearchItemData {
        id: 5,
        command: "-",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("0x11 - 3           // substraction (14)\n0 - 1              // underflowing substraction (-) \nunchecked(0 - 1)   // unchecked overflowing substraction (max_uint)"),
        desc: "Computes the non-underflowing subtraction of two values.",
    },
    SearchItemData {
        id: 6,
        command: "*",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("2 * 3                     // multiplication (6)\nmax_uint * 2              // overflowing multiplication (-)\nunchecked(max_uint * 2)   // unchecked overflowing multiplication (max_uint - 1)"),
        desc: "Computes the non-overflowing multiplication of two values.",
    },
    SearchItemData {
        id: 7,
        command: "/",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("9 / 3    // division (3)\n10 / 3   // division rounded down due to integer math (3)"),
        desc: "Computes the division of two values. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        id: 8,
        command: "%",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("9 % 3    // modulus\n10 % 3   // modulus (1)"),
        desc: "Computes the modulus of two values.",
    },
    SearchItemData {
        id: 9,
        command: "**",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("2 ** 8                // power (256)\n2 ** 256              // overflowing power (-)\nunchecked(2 ** 256)   // unchecked overflowing power"),
        desc: "Computes the power of two values.",
    },
    SearchItemData {
        id: 10,
        command: ">>",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("5 >> 1   // right shift (2)"),
        desc: "Right shift a number by n bits",
    },
    SearchItemData {
        id: 11,
        command: "<<",
        c_type: CommandType::Operation,
        alias: None,
        params: None,
        example: Some("5 << 1   // left shift (10)"),
        desc: "Left shift a number by n bits",
    },
    // END: OPERATION COMMANDS
    // START: CONVERSION COMMANDS
    SearchItemData {
        id: 12,
        command: "EVM gas units",
        c_type: CommandType::Conversion,
        alias: Some("wei, kwei, mwei, gwei, szabo, finney, ether"),
        params: None,
        example: Some("1 ether to gwei  // gas unit conversion (1000000000)"),
        desc: "Converts the input value to the equivalent amount. Follows the pattern: 'unit_from' to 'unit_to'.\nWhen 'unit_to' is bigger than 'unit_from', the result is rounded down.",
    },
    SearchItemData {
        id: 13,
        command: "Time units",
        c_type: CommandType::Conversion,
        alias: Some("seconds, minutes, hours, days, weeks, months, years"),
        params: None,
        example: Some("1 year to seconds  // time unit conversion (31536000)"),
        desc: "Converts the input value to the equivalent amount. Follows the pattern: 'unit_from' to 'unit_to'.\nWhen 'unit_to' is bigger than 'unit_from', the result is rounded down.",
    },
    // END: CONVERSION COMMANDS
    // START: CONSTANTS
    SearchItemData {
        id: 14,
        command: "max_uint",
        c_type: CommandType::Constant,
        alias: Some("max_u256, type(uint256).max"),
        params: None,
        example: Some("max_uint  // (0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)"),
        desc: "Evaluates to the max uint possible with 32 bytes or 256 bits",
    },
    SearchItemData {
        id: 15,
        command: "zero_address",
        c_type: CommandType::Constant,
        alias: Some("address(0), addr(0), address_zero, zadd"),
        params: None,
        example: Some("zero_address  // zero address (0x0000000000000000000000000000000000000000)"),
        desc: "Evaluates to the zero address",
    },
    SearchItemData {
        id: 16,
        command: "now",
        c_type: CommandType::Constant,
        alias: None,
        params: None,
        example: Some("now  // current timestamp"),
        desc: "Evaluates to the current unix timestamp",
    },
    // END: CONSTANTS COMMANDS
    // START: FUNCTION COMMANDS
    SearchItemData {
        id: 17,
        command: "sqrt",
        c_type: CommandType::Function,
        alias: None,
        params: Some("sqrt(num: `uint256`)"),
        example: Some("sqrt(25)   // square root (5)\nsqrt(30)   // square root rounded down due to integer math (5)"),
        desc: "Computes the square root of the input value. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        id: 18,
        command: "root",
        c_type: CommandType::Function,
        alias: None,
        params: Some("root(num: `uint256`, n: `uint256`)"),
        example: Some("root(125, 3)   // N root (5)\nroot(130, 3)   // N root rounded down due to integer math (5)"),
        desc: "Computes the Nth root of the input value. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        id: 19,
        command: "checksum",
        c_type: CommandType::Function,
        alias: Some("addr, address"),
        params: Some("checksum(addr: `bytes20`)"),
        example: Some("// address checksum (0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045)\nchecksum(0xd8da6bf26964af9d7eed9e03e53415d37aa96045)"),
        desc: "Calculates the checksum of an Ethereum address",
    },
    SearchItemData {
        id: 20,
        command: "selector",
        c_type: CommandType::Function,
        alias: None,
        params: Some("selector(fn_sig: `str`)"),
        example: Some("// 4-byte function selector (0xa9059cbb)\nselector(\"transfer(address,uint256)\")"),
        desc: "Returns the 4-byte function selector for Ethereum function signatures.\n The function signature must only the function name followed by the parameter types in parentheses (without parameter names).",
    },
    SearchItemData {
        id: 21,
        command: "keccak256",
        c_type: CommandType::Function,
        alias: Some("hash, sha3"),
        params: Some("keccak256(input: `str`)"),
        example: Some("// keccak hash (0x47173285a8d7..fa254cb01fad)\nkeccak256(\"hello world\")"),
        desc: "Computes the KECCAK-256 hash of the input following the SHA-3 standard.",
    },
    SearchItemData {
        id: 22,
        command: "base64_encode",
        c_type: CommandType::Function,
        alias: Some("b64_encode, b64encode"),
        params: Some("base64_encode(input: `str`)"),
        example: Some("// base64 encode ('aGVsbG8gd29ybGQ=')\nb64_encode(\"hello world\")"),
        desc: "Encodes the input string into Base64 format",
    },
    SearchItemData {
        id: 23,
        command: "base64_decode",
        c_type: CommandType::Function,
        alias: Some("b64_decode, b64decode"),
        params: Some("base64_decode(input: `str`)"),
        example: Some("// base64 decode ('hello world')\nb64_decode(\"aGVsbG8gd29ybGQ=\")"),
        desc: "Decodes the Base64 encoded string back into plain text",
    },
    SearchItemData {
        id: 24,
        command: "abi_encode",
        c_type: CommandType::Function,
        alias: None,
        params: Some("abi_encode(fn_sig: `str`, params: `str`)"),
        example: Some("// abi encode without function selector:\n// 0x\n// 000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045\n// 0000000000000000000000000000000000000000000000000000000000000001\n \nabi_encode(\n  \"transfer(address,uint256)\",   // fn_sig\n  \"0xd8da6bf2..7aa96045, 1\"      // params\n)"),
        desc: "ABI encodes the arguments and outputs the corresponding calldata without the function selector",
    },
    SearchItemData {
        id: 25,
        command: "abi_encode_with_selector",
        c_type: CommandType::Function,
        alias: Some("abi_encode_with_sig"),
        params: Some("abi_encode_with_selector(fn_sig: `str`, params: `str`)"),
        example: Some("// abi encode with function selector:\n// 0x\n// a9059cbb\n// 000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045\n// 0000000000000000000000000000000000000000000000000000000000000001\n \nabi_encode(\n  \"transfer(address,uint256)\",   // fn_sig\n  \"0xd8da6bf2..7aa96045, 1\"      // params\n)"),
        desc: "ABI encodes the arguments and outputs the corresponding calldata with the function selector",
    },
    SearchItemData {
        id: 26,
        command: "abi_decode",
        c_type: CommandType::Function,
        alias: None,
        params: Some("abi_decode(fn_sig: `str`, calldata: `str`)"),
        example: Some("// abi decode:\n//   fn_selector: \"0xa9059cbb\"\n//   address: \"0xd8da6bf26964af9d7eed9e03e53415d37aa96045\"\n//   uint256: \"0x1\"\n \nabi_decode(\n  \"transfer(address,uint256)\",        // fn_sig\n  \"0xa9059cbb000000000000..0000001\"   // calldata\n)"),
        desc: "Decodes calldata given a function signature. Automatically identifies the 8-byte function selectors if present.",
    },
    SearchItemData {
        id: 27,
        command: "debug",
        c_type: CommandType::Function,
        alias: None,
        params: Some("debug(calldata: `str`)"),
        example: Some("// pretty prints calldata in 32-byte words + function selector:\n//   0x\n//   a9059cbb\n//   000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045\n//   0000000000000000000000000000000000000000000000000000000000000001\n \ndebug(\"0xa9059cbb000000000000..0000001\")"),
        desc: "Pretty prints calldata in 32-byte words. Automatically identifies 8-byte function selectors if present.",
    },
    SearchItemData {
        id: 28,
        command: "uppercase",
        c_type: CommandType::Function,
        alias: Some("upper"),
        params: Some("uppercase(input: `str`)"),
        example: Some("upper(\"hello\")  // upper case ('HELLO')"),
        desc: "Converts a string to upper case",
    },
    SearchItemData {
        id: 29,
        command: "lowercase",
        c_type: CommandType::Function,
        alias: Some("lower"),
        params: Some("lowercase(input: `str`)"),
        example: Some("lower(\"WORLD\")  // lower case ('world')"),
        desc: "Converts a string to lower case",
    },
    SearchItemData {
        id: 30,
        command: "len",
        c_type: CommandType::Function,
        alias: Some("chars"),
        params: Some("len(input: `str`)"),
        example: Some("len(\"foo bar\")  // count all characters (7)"),
        desc: "Returns the length of a string",
    },
    SearchItemData {
        id: 31,
        command: "count",
        c_type: CommandType::Function,
        alias: None,
        params: Some("count(input: `str`, substr: `str`)"),
        example: Some("count(\"foo bar\", \"o\")  // count input character (2)"),
        desc: "Counts occurrences of a substring within a string",
    },
    SearchItemData {
        id: 32,
        command: "left_pad",
        c_type: CommandType::Function,
        alias: Some("lpad"),
        params: Some("left_pad(input: `str`, length: `uint8`)"),
        example: None,
        desc: "Pads a string the to the left, with a zeros, to a specified length",
    },
    SearchItemData {
        id: 33,
        command: "right_pad",
        c_type: CommandType::Function,
        alias: Some("rpad"),
        params: Some("right_pad(input: `str`, length: `uint8`)"),
        example: None,
        desc: "Pads a string the to the right, with a zeros, to a specified length",
    },
    SearchItemData {
        id: 34,
        command: "format_ether",
        c_type: CommandType::Function,
        alias: None,
        params: Some("format_ether(input: `uint256`)"),
        example: Some("format_ether(1e18)  // format with 18 decimal places ('1.000000000000000000')"),
        desc: "Formats the input number with 18 decimal places. Since floating point math is not supported, outputs a string.",
    },
    SearchItemData {
        id: 35,
        command: "format_uints",
        c_type: CommandType::Function,
        alias: None,
        params: Some("format_uints(input: `uint256`, decimals: `uint8`)"),
        example: Some("format_uints(123456, 4)  // format with n decimal places ('12.3456')"),
        desc: "Formats the input number with a specified number of decimals. Since floating point math is not supported, outputs a string.",
    },
    SearchItemData {
        id: 36,
        command: "unix",
        c_type: CommandType::Function,
        alias: None,
        params: Some("unix(year: `uint16`, month: `uint8`, day: `uint8`, hour?: `uint8`, minute?: `uint8`, second?: `uint8`)\n\u{00a0}\nunix(timestamp: `uint32`)"),
        example: Some("// unix timestamp, comma separated (1704067199)\nunix(2023, 12, 31, 23, 59, 59)\n \n// unix timestamp, YYYY-MM-DDTHH:mm:ss (1704067199)\nunix(2023-12-31T23:59:59)\n \n// formatted timestamp from unix ('2023-12-31 23:59:59')\nunix(1704067199)\n \n// custom formatted timestamp from unix ('2023-12-31T23:59:59')\nunix(1704067199, \"%Y-%m-%dT%H:%M:%S\")"),
        desc: "Bidirectional function:\n- Converts a unix timestamp to a human-readable date. Accepts a second argument for the date format.\n- Converts strings of comma-separated values or '%Y-%m-%d %H:%M:%S' format to a unix timestamp.",
    },
    SearchItemData {
        id: 37,
        command: "get_price_from_tick",
        c_type: CommandType::Function,
        alias: Some("get_price, price_from_tick, price_at_tick"),
        params: Some("get_price_from_tick(\u{00a0}\u{00a0}\u{00a0}\n\u{00a0}\u{00a0}tick: `int24`,\n\u{00a0}\u{00a0}decimals0: `uint8`,\n\u{00a0}\u{00a0}decimals1: `uint8`,\n\u{00a0}\u{00a0}token0?: `bool`\n)"),
        example: Some("// get price from tick (\"1 token1 : 1540.921115 token0\")\nget_price_from_tick(202919, false, 6, 18)"),
        desc: "Computes the price of a Uniswap V3 pool (in token0 or token1) given a tick value and the token decimals.",
    },
    SearchItemData {
        id: 38,
        command: "get_tick_from_sqrt_ratio",
        c_type: CommandType::Function,
        alias: Some("get_tick, tick_from_sqrt_ratio, tick_from_sqrt_x96"),
        params: Some("get_tick_from_sqrt_ratio(sqrt_ratio: `uint160`)"),
        example: Some("// get tick from sqrt ratio (-887272)\nget_tick_from_sqrt_ratio(4295128739)"),
        desc: "Computes the tick of a Uniswap V3 pool given a square root of price as a Q64.96.",
    },
    SearchItemData {
        id: 39,
        command: "get_sqrt_ratio_from_tick",
        c_type: CommandType::Function,
        alias: Some("get_sqrt_ratio, get_sqrt_x96, sqrt_ratio_from_tick, sqrt_x96_from_tick"),
        params: Some("get_sqrt_ratio_from_tick(tick: `int24`)"),
        example: Some("// get sqrt ratio from tick (4295128739)\nget_sqrt_ratio_from_tick(-887272)"),
        desc: "Computes the square root of price as a Q64.96 give the tick of a Uniswap V3 pool.",
    },
    SearchItemData {
        id: 40,
        command: "get_liquidity_from_total_amount1",
        c_type: CommandType::Function,
        alias: Some("get_liquidity, liquidity_from_total_amount1"),
        params: Some("get_liquidity_from_total_amount1(\u{00a0}\u{00a0}\u{00a0}\n\u{00a0}\u{00a0}total_amount1: `uint256`,\n\u{00a0}\u{00a0}sqrt_price: `uint160`,\n\u{00a0}\u{00a0}sqrt_pa: `uint160`,\n\u{00a0}\u{00a0}sqrt_pb: `uint160`\n)"),
        example: Some("// get liquidity from total amount1 (44928398530981124971653892)\nget_liquidity_from_total_amount1(\n  1e6,          // total_amount1\n  5317859378,   // sqrt_price\n  4295128739,   // sqrt_pa\n  6178424788    // sqrt_pb\n)"),
        desc: "Computes the equivalent liquidity of a Uniswap V3 range given the pool's sqrtPrice, and the range's amount1, sqrtPa, and sqrtPb.",
    },
    // TODO: doc examples
    SearchItemData {
        id: 41,
        command: "get_amount0_from_range",
        c_type: CommandType::Function,
        alias: Some("get_amount0, amount0_from_range"),
        params: Some("get_amount0_from_range(\u{00a0}\u{00a0}\u{00a0}\n\u{00a0}\u{00a0}liquidity: `uint128`,\n\u{00a0}\u{00a0}sqrt_price: `uint160`,\n\u{00a0}\u{00a0}sqrt_pa: `uint160`,\n\u{00a0}\u{00a0}sqrt_pb: `uint160`\n)"),
        example: Some("// get amount0 from liquidity\nget_amount0_from_range(\n  44928398530981124971653892,\n  5317859378,\n  4295128739,\n  6178424788\n)"),
        desc: "Computes the equivalent amount0 of a Uniswap V3 range given the pool's sqrtPrice, and the range's liquidity, sqrtPa, and sqrtPb.",
    },
    SearchItemData {
        id: 42,
        command: "get_amount1_from_range",
        c_type: CommandType::Function,
        alias: Some("get_amount1, amount1_from_range"),
        params: Some("get_amount1_from_range(\u{00a0}\u{00a0}\u{00a0}\n\u{00a0}\u{00a0}liquidity: `uint128`,\n\u{00a0}\u{00a0}sqrt_price: `uint160`,\n\u{00a0}\u{00a0}sqrt_pa: `uint160`,\n\u{00a0}\u{00a0}sqrt_pb: `uint160`\n)"),
        example: Some("// get amount1 from liquidity\nget_amount0_from_range(\n  44928398530981124971653892,\n  5317859378,\n  4295128739,\n  6178424788\n)"),
        desc: "Computes the equivalent amount1 of a Uniswap V3 range given the pool's sqrtPrice, and the range's liquidity, sqrtPa, and sqrtPb.",
    },
    SearchItemData {
        id: 43,
        command: "get_token0",
        c_type: CommandType::Function,
        alias: Some("token0"),
        params: None,
        example: Some("// get smallest address (0x6a023ccd1ff6f2045c3309768ead9e68f978f6e1)\nget_token0(\n  '0x9c58bacc331c9aa871afd802db6379a98e80cedb',\n  '0x6a023ccd1ff6f2045c3309768ead9e68f978f6e1'\n)"),
        desc: "Retuns the token0 of a Uniswap V3 pool.",
    },
    SearchItemData {
        id: 44,
        command: "get_token1",
        c_type: CommandType::Function,
        alias: Some("token1"),
        params: None,
        example: Some("// get biggest address (0x9c58bacc331c9aa871afd802db6379a98e80cedb)\nget_token1(\n  '0x9c58bacc331c9aa871afd802db6379a98e80cedb',\n  '0x6a023ccd1ff6f2045c3309768ead9e68f978f6e1'\n)"),
        desc: "Retuns the token1 of a Uniswap V3 pool.",
    },
    SearchItemData {
        id: 45,
        command: "get_quote_from_tick",
        c_type: CommandType::Function,
        alias: Some("get_quote, quote_from_tick, quote_at_tick"),
        params: Some("get_quote_from_tick(tick: `int24`, token0?: `bool`, decimals0: `uint8`, decimals1: `uint8`)"),
        example: Some("// get quote from tick (1540921115)\nget_quote_from_tick(202919, false, 6, 18)"),
        desc: "Computes the quote of a Uniswap V3 pool (in token0 or token1) given a tick value and the token decimals.",
    },
    SearchItemData {
        id: 46,
        command: "get_sqrt_ratio_from_price",
        c_type: CommandType::Function,
        alias: Some("get_sqrt_x96_from_price, sqrt_from_price"),
        params: Some("get_sqrt_ratio_from_price(price: `float128`)"),
        example: Some("get_sqrt_ratio_from_price(0.0003)   // sqrt priceX96 (1372272022653615573403678740)"),
        desc: "Computes the sqrtPrice of a Uniswap V3 pool given a price (ratio between tokens, accounting for its decimals).",
    },
    SearchItemData {
        id: 47,
        command: "get_tick_from_price",
        c_type: CommandType::Function,
        alias: Some("tick_from_price"),
        params: Some("get_tick_from_price(price: `float128`)"),
        example: Some("get_tick_from_price(0.0003)   // tick (-81122)"),
        desc: "Computes the tick of a Uniswap V3 pool given a price (ratio between tokens, accounting for its decimals).\n Note that the tick may need to be adjusted basead on the pool's tick spacing.",
    },
    SearchItemData {
        id: 48,
        command: "get_pool_tick",
        c_type: CommandType::Function,
        alias: Some("get_tick_from_spacing, get_tick_with_spacing"),
        params: Some("get_pool_tick(tick: `i32`, tick_spacing: `u24`)"),
        example: Some("get_pool_tick(-887272, 10)   // tick (-887270)\nget_pool_tick(887272, 200)   // tick (887200)"),
        desc: "Computes the tick of a Uniswap V3 pool given a price (ratio between tokens, accounting for its decimals).\n Note that the tick may need to be adjusted basead on the pool's tick spacing.",
    },
    // TODO: fix calculation
    // SearchItemData {
    //     id: 48,
    //     command: "get_upper_tick_and_sqrt_ratio",
    //     c_type: CommandType::Function,
    //     alias: Some("get_upper_sqrt_ratio_and_tick, get_upper_both"),
    //     params: Some("get_upper_tick_and_sqrt_ratio(\u{00a0}\u{00a0}\u{00a0}\n\u{00a0}\u{00a0}liquidity: `uint128`,\n\u{00a0}\u{00a0}use_amount0: `uint256`,\n\u{00a0}\u{00a0}sqrt_price: `uint160`\n)"),
    //     example: None,
    //     desc: "Computes the tick of a Uniswap V3 pool given a price (ratio between tokens, accounting for its decimals).\n Note that the tick may need to be adjusted basead on the pool's tick spacing.",
    // },
    // END: FUNCTION COMMANDS
];
