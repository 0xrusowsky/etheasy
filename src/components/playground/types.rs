use crate::parser::types::result::ParseResult;

use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, File, FileReader, HtmlElement, Url};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BlockInput {
    value: String,
    height: i32,
}

impl BlockInput {
    pub fn new(value: String, height: i32) -> Self {
        Self { value, height }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockState {
    id: String,
    result: ParseResult,
}

impl BlockState {
    pub fn from_id(id: usize) -> Self {
        Self {
            id: format!("block_{}", id),
            result: ParseResult::NAN,
        }
    }

    pub fn update_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn update_result(&mut self, result: ParseResult) {
        self.result = result;
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_result(&self) -> ParseResult {
        self.result.clone()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NotebookBlock {
    label: String,
    input: String,
    height: i32,
    result: ParseResult,
}

impl NotebookBlock {
    pub fn new(input: &BlockInput, state: &BlockState) -> Self {
        Self {
            label: state.id.clone(),
            input: input.value.clone(),
            height: input.height,
            result: state.result.clone(),
        }
    }
}

// Alias for Vec<NotebookBlock>
pub type Notebook = Vec<NotebookBlock>;

// Trait to implement methods for Notebook
trait NotebookMethods {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Self;
    fn into_blocks(self) -> (Vec<BlockState>, Vec<BlockInput>);
}

impl NotebookMethods for Notebook {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    fn into_blocks(self) -> (Vec<BlockState>, Vec<BlockInput>) {
        let mut block_states = Vec::with_capacity(self.len());
        let mut block_inputs = Vec::with_capacity(self.len());

        for block in self {
            block_states.push(BlockState {
                id: block.label,
                result: block.result,
            });
            block_inputs.push(BlockInput {
                value: block.input,
                height: block.height,
            });
        }

        (block_states, block_inputs)
    }
}

// Function to create download callback
pub fn download_notebook(notebook: Notebook) {
    let json = notebook.to_json();
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(&json));

    let blob = Blob::new_with_str_sequence_and_options(
        &array,
        BlobPropertyBag::new().type_("application/json"),
    )
    .unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", "etheasy_notebook.json")
        .unwrap();
    let a = a.dyn_into::<HtmlElement>().unwrap();
    a.click();

    Url::revoke_object_url(&url).unwrap();
}

pub fn load_notebook(
    file: File,
    success_callback: Callback<(Vec<BlockState>, Vec<BlockInput>)>,
    error_callback: Callback<()>,
) {
    let file_reader = std::rc::Rc::new(std::cell::RefCell::new(FileReader::new().unwrap()));
    let reader_clone = file_reader.clone();

    let onloadend = Closure::wrap(Box::new(move || {
        let result = reader_clone.borrow().result().unwrap().as_string().unwrap();
        match serde_json::from_str::<Notebook>(&result) {
            Ok(notebook) => {
                let (states, inputs) = notebook.into_blocks();
                success_callback.emit((states, inputs));
            }
            Err(_) => {
                gloo_console::log!("Error parsing import file!");
                error_callback.emit(());
            }
        };
    }) as Box<dyn FnMut()>);

    file_reader
        .borrow_mut()
        .set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
    file_reader.borrow().read_as_text(&file).unwrap();
    onloadend.forget();
}
