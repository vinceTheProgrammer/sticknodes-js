mod color;
mod macros;
mod node;
mod node_options;
mod polyfill;
mod polyfill_options;
mod stickfigure;
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;

#[cfg(feature = "console_error_panic_hook")]
use std::panic;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
