#[derive(Debug, Clone, PartialEq)]
pub struct SearchItemData {
    pub command: &'static str,
    pub c_type: CommandType,
    pub c_alias: Option<&'static str>,
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
            CommandType::Input => "input",
            CommandType::Function => "function",
            CommandType::Constant => "constant",
            CommandType::Operation => "operation",
            CommandType::Conversion => "conversion",
        }
    }
}

pub static SEARCH_ITEMS: &[SearchItemData; 37] = &[
    SearchItemData {
        command: "variable",
        c_type: CommandType::Input,
        c_alias: None,
        desc: "Block results are stored in the app state, and can be referenced in other blocks.\nBy default names follow 'block_x' notation, but you can be renamed by modifying the block label.\nVariables don't need to be wrapped in quotes, and will evaluate to their corresponding value.",
    },
    SearchItemData {
        command: "uint256",
        c_type: CommandType::Input,
        c_alias: None,
        desc: "EVM word that consists of up to 32 bytes or 256 bits representing an unisgned integer.\nAccepts decimal, hex, or binary inputs. Also supports scientific notation for decimals.\nDue to its integer nature, it doesn't support floating point numbers.\nDue to its unsigned nature, it doesn't support negative numbers.",
    },
    SearchItemData {
        command: "string",
        c_type: CommandType::Input,
        c_alias: None,
        desc: "Strings must be wrapped around quotation marks. Both single and double quotes are accepted.",
    },
    SearchItemData {
        command: "unchecked",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Enables unchecked math for any calculation performed inside its brackets.",
    },
    SearchItemData {
        command: "+",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the non-overflowing addition of two values.",
    },
    SearchItemData {
        command: "-",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the non-underflowing substraction of two values.",
    },
    SearchItemData {
        command: "*",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the non-overflowing multiplication of two values.",
    },
    SearchItemData {
        command: "/",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the division of two values. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        command: "%",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the modulus of two values.",
    },
    SearchItemData {
        command: "**",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Computes the power of two values.",
    },
    SearchItemData {
        command: ">>",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Right shift a number by n bits",
    },
    SearchItemData {
        command: "<<",
        c_type: CommandType::Operation,
        c_alias: None,
        desc: "Left shift a number by n bits",
    },
    SearchItemData {
        command: "sqrt",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Computes the square root of the input value. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        command: "root",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Computes the nth root of the input value. The result is rounded down to the nearest integer.",
    },
    SearchItemData {
        command: "checksum",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Calculates the checksum of an Ethereum address",
    },
    SearchItemData {
        command: "selector",
        c_type: CommandType::Function,
        c_alias: Some("address, addr"),
        desc: "Returns the 4-byte function selector for Ethereum function signatures",
    },
    SearchItemData {
        command: "keccak256",
        c_type: CommandType::Function,
        c_alias: Some("sha3"),
        desc: "Computes the KECCAK-256 hash of the input following the SHA-3 standard.",
    },
    SearchItemData {
        command: "base64_encode",
        c_type: CommandType::Function,
        c_alias: Some("b64_encode, b64encode"),
        desc: "Encodes the input string into Base64 format",
    },
    SearchItemData {
        command: "base64_decode",
        c_type: CommandType::Function,
        c_alias: Some("b64_decode, b64decode"),
        desc: "Decodes the Base64 encoded string back into plain text",
    },
    SearchItemData {
        command: "abi_encode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "ABI encodes the arguments and outputs the corresponding calldata without the function selector",
    },
    SearchItemData {
        command: "abi_encode_with_selector",
        c_type: CommandType::Function,
        c_alias: Some("abi_encode_with_sig"),
        desc: "ABI encodes the arguments and outputs the corresponding calldata with the function selector",
    },
    SearchItemData {
        command: "abi_decode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Decodes calldata given a function signature. Automatically identifies the 8-byte function selectors if present.",
    },
    SearchItemData {
        command: "debug",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Pretty prints calldata in 32-byte words. Automatically identifies 8-byte function selectors if present.",
    },
    SearchItemData {
        command: "uppercase",
        c_type: CommandType::Function,
        c_alias: Some("upper"),
        desc: "Converts a string to upper case",
    },
    SearchItemData {
        command: "lowercase",
        c_type: CommandType::Function,
        c_alias: Some("lower"),
        desc: "Converts a string to lower case",
    },
    SearchItemData {
        command: "len",
        c_type: CommandType::Function,
        c_alias: Some("chars"),
        desc: "Returns the length of a string",
    },
    SearchItemData {
        command: "count",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Counts occurrences of a substring within a string",
    },
    SearchItemData {
        command: "left_pad",
        c_type: CommandType::Function,
        c_alias: Some("lpad"),
        desc: "Pads a string the to the left, with a zeros, to a specified length",
    },
    SearchItemData {
        command: "right_pad",
        c_type: CommandType::Function,
        c_alias: Some("rpad"),
        desc: "Pads a string the to the right, with a zeros, to a specified length",
    },
    SearchItemData {
        command: "format_ether",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Formats the input number with 18 decimal places. Since floating point math is not supported, outputs a string.",
    },
    SearchItemData {
        command: "format_uints",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Formats the input number with a specified number of decimals. Since floating point math is not supported, outputs a string.",
    },
    SearchItemData {
        command: "unix",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Bidirectional function:\n- Converts a unix timestamp to a human-readable date. Accepts a second argument for the date format.\n- Converts strings of comma-separated values or '%Y-%m-%d %H:%M:%S' format to a unix timestamp.",
    },
    SearchItemData {
        command: "EVM gas units",
        c_type: CommandType::Conversion,
        c_alias: Some("wei, kwei, mwei, gwei, szabo, finney, ether"),
        desc: "Converts the input value to the equivalent amount. Follows the pattern: 'unit_from' to 'unit_to'.\nWhen 'unit_to' is bigger than 'unit_from', the result is rounded down.",
    },
    SearchItemData {
        command: "Time units",
        c_type: CommandType::Conversion,
        c_alias: Some("seconds, minutes, hours, days, weeks, months, years"),
        desc: "Converts the input value to the equivalent amount. Follows the pattern: 'unit_from' to 'unit_to'.\nWhen 'unit_to' is bigger than 'unit_from', the result is rounded down.",
    },
    SearchItemData {
        command: "max_uint",
        c_type: CommandType::Constant,
        c_alias: Some("max_u256, type(uint256).max"),
        desc: "Evaluates to the max uint possible with 32 bytes or 256 bits",
    },
    SearchItemData {
        command: "zero_address",
        c_type: CommandType::Constant,
        c_alias: Some("address(0), addr(0), address_zero, zadd"),
        desc: "Evaluates to the zero address",
    },
    SearchItemData {
        command: "now",
        c_type: CommandType::Constant,
        c_alias: None,
        desc: "Evaluates to the current unix timestamp",
    },
];
