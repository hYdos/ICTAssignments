use js_sys::JsString;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement, WebGlRenderingContext};

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn get_document() -> Result<Document, JsValue> {
    Ok(web_sys::window().ok_or(JsString::from("Window not present"))?
        .document().ok_or(JsString::from("Document not present"))?)
}

pub fn get_gl(canvas: &str) -> Result<WebGlRenderingContext, JsValue> {
    Ok(get_canvas(canvas)?
        .get_context("webgl")?.ok_or(JsString::from("WebGL context not found"))?
        .dyn_into::<WebGlRenderingContext>()?)
}

pub fn get_canvas(canvas: &str) -> Result<HtmlCanvasElement, JsValue> {
    Ok(get_document()?
        .get_element_by_id(canvas).ok_or(JsString::from("Canvas element not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?)
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
