mod utils;

// in this file we can use #[wasm_bindgen] only because it is brought into scope by the prelude.
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// rust -> js
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// js -> rust
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}
