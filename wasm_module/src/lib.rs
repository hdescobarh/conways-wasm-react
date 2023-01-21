#[cfg(test)]
pub mod tests;
pub mod universe;
pub mod utils;
use js_sys::Array;
use std::process::abort;
use wasm_bindgen::prelude::*;
// use 2-glider_mess to make tests in web

#[wasm_bindgen]
pub struct Simulation {
    current_universe: universe::Universe,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new() -> Self {
        Self {
            current_universe: universe::Universe::new(4, 4, vec![false; 16]),
        }
    }

    pub fn initialize_universe(&mut self, height: usize, width: usize, init_array: Array) {
        let init_space: Vec<bool> = init_array
            .iter()
            .map(|value| value.as_bool().unwrap_or_else(|| abort()))
            .collect();
        self.current_universe = universe::Universe::new(height, width, init_space);
    }

    pub fn time_step(&mut self) -> usize {
        self.current_universe.time_step();
        *self.current_universe.get_age()
    }

    pub fn get_cell_value(&self, index: usize) -> bool {
        *self.current_universe.get_single_value_by_raw_index(index)
    }
}
