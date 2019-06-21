// The src/lib.rs file is the root of the Rust crate that we are compiling to
// WebAssembly. It uses wasm-bindgen to interface with JavaScript. it imports
// the window.alert JavaScript function, and exports the greet Rust function,
// which alerts a greeting message.

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Imports the `window.alert` function from the Web.
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
