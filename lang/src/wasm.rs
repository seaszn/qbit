#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use serde::{Deserialize, Serialize};

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use crate::parser::{Diagnostic, ParseError, ParseResult, Parser};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[derive(Serialize, Deserialize)]
pub struct WasmResult {
    success: bool,
    diagnostics: Vec<Diagnostic>,
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl From<Result<ParseResult, ParseError>> for WasmResult {
    fn from(value: Result<ParseResult, ParseError>) -> Self {
        match value {
            Ok(result) => WasmResult {
                success: true,
                diagnostics: result.diagnositcs().to_vec(),
            },
            Err(error) => WasmResult {
                success: false,
                diagnostics: vec![Diagnostic::from(error)],
            },
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn init_panic() {
    console_error_panic_hook::set_once();
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn parse_code(source: &str) -> JsValue {
    let parse_result = Parser::parse_src(source);
    let wasm_result = WasmResult::from(parse_result);

    serde_wasm_bindgen::to_value(&wasm_result).unwrap()
}