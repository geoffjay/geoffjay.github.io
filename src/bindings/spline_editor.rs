use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/scenes/spline-editor.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn init();

    #[wasm_bindgen]
    pub fn render();
}
