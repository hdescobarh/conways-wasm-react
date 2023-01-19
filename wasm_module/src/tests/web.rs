use crate::*;

// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
// the tests must be in the root of the crate, or within a pub mod
use wasm_bindgen_test::*;
//This macro macro is used to indicate that the test is intended to be executed in a web browser (and not in other enviroment; Node.js for example)
wasm_bindgen_test_configure!(run_in_browser);
// This attribute defines a test accessible to WebAssembly and headless web browser testing
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn simulation_initialization() {
    let simulation = Simulation::new();
    //simulation.initialize_universe(height, width, init_array);
    let init_space = vec![
        false, true, false, false, false, true, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    // need to create an Array JS
}
