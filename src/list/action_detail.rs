use crate::list::ListIndex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Object;

#[derive(Debug)]
pub struct ActionDetail {
    index: ListIndex,
}

impl From<JsValue> for ActionDetail {
    fn from(value: JsValue) -> Self {
        let detail = value.unchecked_into::<ActionDetailJs>();
        let index = ListIndex::from(detail.index());
        Self {
            index,
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type ActionDetailJs;

    #[wasm_bindgen(method, getter)]
    pub fn index(this: &ActionDetailJs) -> JsValue;
}
