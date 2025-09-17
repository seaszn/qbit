mod result;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::parser::Parser;

#[cfg(feature = "wasm")]
use result::{VsCodeError, VsCodeResult};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_code(source: &str) -> JsValue {
    let parse_result = crate::parser::Parser::parse_src(source);
    let vscode_result = VsCodeResult::from(parse_result);

    serde_wasm_bindgen::to_value(&vscode_result).unwrap()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_syntax(source: &str) -> JsValue {
    let errors: Vec<VsCodeError> = match Parser::parse_src(source) {
        Ok(_) => vec![],
        Err(error) => vec![VsCodeError::from(error)],
    };

    serde_wasm_bindgen::to_value(&errors).unwrap()
}
