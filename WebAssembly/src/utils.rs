use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, WebGlRenderingContext};

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn get_document() -> Document {
    return web_sys::window().unwrap().document().unwrap();
}

pub fn get_gl(canvas: HtmlCanvasElement) -> Result<WebGlRenderingContext, JsValue> {
    return Ok(canvas
        .get_context("webgl")?
        .ok_or(JsValue::null())?
        .dyn_into::<WebGlRenderingContext>()?);
}

pub trait JsStringExtensions {
    fn to_rust_string(&self) -> String;
}

impl JsStringExtensions for JsString {
    fn to_rust_string(&self) -> String {
        self.into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}
