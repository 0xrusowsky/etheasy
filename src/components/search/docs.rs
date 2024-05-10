#[derive(Debug, Clone, PartialEq)]
pub struct SearchItemData {
    pub command: &'static str,
    pub c_type: CommandType,
    pub c_alias: Option<&'static str>,
    pub desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Function,
    Constant,
}

impl CommandType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CommandType::Function => "function",
            CommandType::Constant => "constant",
        }
    }
}

pub static SEARCH_ITEMS: &[SearchItemData; 20] = &[
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
    SearchItemData {
        command: "unchecked",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Enables unchecked math for any calculation performed inside its brackets.",
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
        c_alias: None,
        desc: "Returns the 4-byte function selector for Ethereum function signatures",
    },
    SearchItemData {
        command: "keccak256",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Computes the KECCAK-256 hash of the input",
    },
    SearchItemData {
        command: "b64_encode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Encodes the input string into Base64 format",
    },
    SearchItemData {
        command: "b64_decode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Decodes the Base64 encoded string back into plain text",
    },
    SearchItemData {
        command: "abi_encode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "ABI encodes the arguments without the function selector",
    },
    SearchItemData {
        command: "abi_encode_with_selector",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "ABI encodes the arguments with the function selector",
    },
    SearchItemData {
        command: "abi_decode",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Decodes ABI-encoded data given a function signature",
    },
    SearchItemData {
        command: "debug",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Pretty prints calldata in 32-byte words including function selector",
    },
    SearchItemData {
        command: "upper",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Converts a string to upper case",
    },
    SearchItemData {
        command: "lower",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Converts a string to lower case",
    },
    SearchItemData {
        command: "len",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Returns the length of a string",
    },
    SearchItemData {
        command: "count",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Counts occurrences of a substring in a string",
    },
    SearchItemData {
        command: "format_ether",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Formats an amount in wei to ether with decimal places",
    },
    SearchItemData {
        command: "format_uints",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Formats a large number into a string with a specified number of decimals",
    },
    SearchItemData {
        command: "unix",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Handles various operations with Unix timestamps",
    },
];
