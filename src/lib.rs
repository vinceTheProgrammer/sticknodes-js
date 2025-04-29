mod color;
mod node;
mod polyfill;
mod stickfigure;
mod node_options;
mod polyfill_options;
mod macros;
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