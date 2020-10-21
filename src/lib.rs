mod utils;

use log::info;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    utils::init_log();
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    if name == "panic" {
        panic!("force panic!");
    }
    let greet_str = format!("Hello, {}!", name);
    alert(&greet_str);
    info!("{}", &greet_str);
}
