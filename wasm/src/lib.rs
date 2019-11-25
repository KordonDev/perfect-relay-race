mod utils;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Run {
    distance: u32,
    duration: u32,
}

#[wasm_bindgen]
impl Run {
    pub fn new(distance: u32, duration: u32) -> JsValue {
        utils::set_panic_hook();
        let run = Run {
            distance,
            duration
        };
        JsValue::from_serde(&run).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    runs: Vec<Run>,
}

#[wasm_bindgen]
impl Person {
    pub fn new(name: String, value_runs: &JsValue) -> JsValue {
        utils::set_panic_hook();
        let runs: Vec<Run> = value_runs.into_serde().unwrap();
        let person = Person {
            name,
            runs,
        };
        JsValue::from_serde(&person).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Result {
    name: String,
    run: Run,
}



#[wasm_bindgen]
pub fn print_run(val: &JsValue) {
    let example: Run = val.into_serde().unwrap();
    log!("Run 1: distance {}, duration {}", example.distance, example.duration);
}

#[wasm_bindgen]
pub fn print_person(val: &JsValue) {
    let example: Person = val.into_serde().unwrap();
    log!("Person 1: runs {}", example.runs.len());
    log!("Person 1: name {}", example.name);
}

#[wasm_bindgen]
pub fn calculate(val: &JsValue) -> JsValue {
    utils::set_panic_hook();
    let persons: Vec<Person> = val.into_serde().unwrap();
    let mut result_vector: Vec<Result> = Vec::new();

    log!("len: {}", persons.len());

    for i in persons {
        let run_fast: Run = Run {
            distance: i.runs[0].distance,
            duration: i.runs[0].duration,
        };
        let res = Result {
            name: i.name,
            run: run_fast,
        };
        result_vector.push(res);
    }

    JsValue::from_serde(&result_vector).unwrap()
}
