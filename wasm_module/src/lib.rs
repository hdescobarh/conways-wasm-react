mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_unumbers(left: usize, right: usize) -> Option<usize> {
    left.checked_add(right)
}