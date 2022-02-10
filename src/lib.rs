#[cfg(target_arch = "wasm32")]
use eframe::{
    start_web,
    wasm_bindgen::{self, prelude::*, JsValue},
};

mod labs;

pub use labs::LabsApp;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    start_web(canvas_id, Box::new(LabsApp::default()))
}
