use crate::*;

// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html
// the tests must be in the root of the crate, or within a pub mod
use wasm_bindgen_test::*;
//This macro macro is used to indicate that the test is intended to be executed in a web browser (and not in other enviroment; Node.js for example)
wasm_bindgen_test_configure!(run_in_browser);
// This attribute defines a test accessible to WebAssembly and headless web browser testing

#[wasm_bindgen_test]
fn simulation_constructor() {
    let _simulation = Simulation::new();
}

#[wasm_bindgen_test]
fn simulation_initialize_universe() {
    let mut simulation = Simulation::new();

    let init_space = vec![
        false, true, false, false, false, true, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let init_array = {
        let array = Array::new_with_length(25);
        let mut index = 0u32;
        for value in init_space {
            let js_value = if value { JsValue::TRUE } else { JsValue::FALSE };
            array.set(index, js_value);
            index += 1;
        }
        array
    };

    simulation.initialize_universe(5, 5, init_array);
}
#[wasm_bindgen_test]
fn simulation_getters() {
    let mut simulation = Simulation::new();

    let init_space = vec![
        true, true, false, false, false, true, true, false, false, false, false, false, true,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let init_array = {
        let array = Array::new_with_length(25);
        let mut index = 0u32;
        for value in &init_space {
            let js_value = if *value {
                JsValue::TRUE
            } else {
                JsValue::FALSE
            };
            array.set(index, js_value);
            index += 1;
        }
        array
    };

    simulation.initialize_universe(5, 5, init_array);

    assert_eq!(init_space, *simulation.current_universe.get_space_raw());
    assert_eq!(init_space[5], simulation.get_cell_value(5));
}

#[wasm_bindgen_test]
fn simulation_time_step() {
    let mut simulation = Simulation::new();

    let init_space = vec![
        true, true, false, false, false, true, true, false, false, false, false, false, true,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let init_array = {
        let array = Array::new_with_length(25);
        let mut index = 0u32;
        for value in &init_space {
            let js_value = if *value {
                JsValue::TRUE
            } else {
                JsValue::FALSE
            };
            array.set(index, js_value);
            index += 1;
        }
        array
    };

    let space_one = vec![
        true, true, false, false, false, true, false, true, false, false, false, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    simulation.initialize_universe(5, 5, init_array);

    let generation = simulation.time_step();
    assert_eq!(space_one, *simulation.current_universe.get_space_raw());
    assert_eq!(1, generation);
}
