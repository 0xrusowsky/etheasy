use alloy_core::primitives::hex;
use alloy_dyn_abi::FunctionExt;
use alloy_dyn_abi::JsonAbiExt;
use alloy_dyn_abi::{DynSolType, DynSolValue};
use alloy_json_abi::Function;
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

/// Decodes the calldata of the function
pub fn abi_decode_calldata(
    sig: &str,
    calldata: &str,
    input: bool,
    fn_selector: bool,
) -> Result<Vec<DynSolValue>, String> {
    let func = match Function::parse(sig) {
        Ok(func) => func,
        Err(e) => return Err(format!("could not parse function signature: {}", e)),
    };
    let calldata = match hex::decode(calldata) {
        Ok(calldata) => calldata,
        Err(e) => return Err(format!("failed to decode calldata: {}", e)),
    };

    let mut calldata = calldata.as_slice();
    // If function selector is prefixed in "calldata", remove it (first 4 bytes)
    if input && fn_selector && calldata.len() >= 4 {
        calldata = &calldata[4..];
    }

    let res = if input {
        func.abi_decode_input(calldata, false)
    } else {
        func.abi_decode_output(calldata, false)
    };

    match res {
        Ok(res) => {
            if res.is_empty() {
                return Err("no arguments found".to_string());
            }

            Ok(res)
        }
        Err(e) => Err(format!("failed to decode calldata: {}", e)),
    }
}

pub fn abi_process_and_decode_calldata(
    abi: &str,
    calldata: &str,
) -> (Option<FunctionSelector>, Result<Vec<Encodable>, String>) {
    // process abi to get the function signature
    let sig = if abi.starts_with("(") && abi.ends_with(")") {
        format!("dummy_fn{}", abi)
    } else {
        abi.to_string()
    };
    // std data length
    let calldata = if calldata.starts_with("0x") {
        calldata.to_string()
    } else {
        format!("0x{}", calldata)
    };

    let selector = if calldata.len() % 64 == 10 {
        Some(FunctionSelector {
            fn_selector: calldata[..10].to_string(),
        })
    } else {
        if calldata.len() % 64 != 2 {
            return (None, Err("invalid calldata length".to_string()));
        }
        None
    };

    let decoded = abi_decode_calldata(&sig, &calldata, true, selector.is_some());
    (
        selector,
        decoded.map(|vec_dynsol| vec_dynsol.into_iter().map(Encodable::from).collect()),
    )
}

/// Given a function and a vector of string arguments, it proceeds to convert the args to alloy
/// [DynSolValue]s and then ABI encode them.
pub fn encode_function_args(func: &Function, args: Vec<String>) -> Result<Vec<u8>, String> {
    let params: Result<Vec<_>, _> = std::iter::zip(&func.inputs, args.into_iter())
        .map(
            |(input, arg)| match DynSolType::parse(&input.selector_type()) {
                Ok(ty) => match DynSolType::coerce_str(&ty, &arg) {
                    Ok(encoded) => Ok(encoded),
                    Err(e) => return Err(format!("failed to coerce value: {}", e)),
                },
                Err(e) => return Err(format!("failed to parse type: {}", e)),
            },
        )
        .collect();

    let params = params.map_err(|e| format!("failed to prepare function arguments: {}", e))?;
    match func.abi_encode_input(params.as_slice()) {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("failed to encode function arguments: {}", e)),
    }
}

pub fn abi_encode(abi: &str, args: Vec<String>, with_selector: bool) -> Result<String, String> {
    // process abi to get the function signature
    let sig = if abi.starts_with("(") && abi.ends_with(")") {
        format!("dummy_fn{}", abi)
    } else {
        abi.to_string()
    };
    let func = match Function::parse(&sig) {
        Ok(func) => func,
        Err(e) => return Err(format!("could not parse function signature: {}", e)),
    };
    let calldata = match encode_function_args(&func, args) {
        Ok(res) => hex::encode(res),
        Err(e) => return Err(format!("Could not ABI encode the function and arguments. Did you pass in the right types?\nError\n{}", e)),
    };
    if !with_selector {
        Ok(format!("0x{}", &calldata[8..]))
    } else {
        Ok(format!("0x{calldata}"))
    }
}

#[derive(Debug)]
pub struct Encodable(DynSolValue);

impl From<DynSolValue> for Encodable {
    fn from(dyn_sol_value: DynSolValue) -> Self {
        Encodable(dyn_sol_value)
    }
}

impl Serialize for Encodable {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            DynSolValue::Bool(v) => {
                let mut state = serializer.serialize_struct("bool", 1)?;
                state.serialize_field("bool", v)?;
                state.end()
            }
            DynSolValue::Int(v, u) => {
                let type_name = Box::leak(Box::new(format!("int{}", u)));
                let mut state = serializer.serialize_struct(type_name, 1)?;
                state.serialize_field(type_name, v)?;
                state.end()
            }
            DynSolValue::Uint(v, u) => {
                let type_name = Box::leak(Box::new(format!("uint{}", u)));
                let mut state = serializer.serialize_struct(type_name, 1)?;
                state.serialize_field(type_name, v)?;
                state.end()
            }
            DynSolValue::FixedBytes(v, u) => {
                let type_name = Box::leak(Box::new(format!("bytes{}", u)));
                let mut state = serializer.serialize_struct(type_name, 1)?;
                state.serialize_field(type_name, v)?;
                state.end()
            }
            DynSolValue::Address(v) => {
                let mut state = serializer.serialize_struct("address", 1)?;
                state.serialize_field("address", v)?;
                state.end()
            }
            DynSolValue::Function(_) => {
                gloo_console::log!("Unexpected function type in serialization.");
                unreachable!("Unexpected function type in serialization")
            }
            DynSolValue::Bytes(v) => {
                let mut state = serializer.serialize_struct("bytes", 1)?;
                state.serialize_field("bytes", v)?;
                state.end()
            }
            DynSolValue::String(v) => {
                let mut state = serializer.serialize_struct("string", 1)?;
                state.serialize_field("string", v)?;
                state.end()
            }
            DynSolValue::Array(v) | DynSolValue::FixedArray(v) | DynSolValue::Tuple(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for elem in v.iter() {
                    seq.serialize_element(&Encodable(elem.clone()))?;
                }
                seq.end()
            }
        }
    }
}

// Define a struct representing the object
#[derive(Serialize)]
pub struct FunctionSelector {
    fn_selector: String,
}
