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
    name: String,
}

#[wasm_bindgen]
impl Run {
    pub fn new(distance: u32, duration: u32, name: String) -> JsValue {
        utils::set_panic_hook();
        let run = Run {
            distance,
            duration,
            name
        };
        JsValue::from_serde(&run).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Person {
    runs: Vec<Run>,
}

#[wasm_bindgen]
impl Person {
    pub fn new(value_runs: &JsValue) -> JsValue {
        utils::set_panic_hook();
        let runs: Vec<Run> = value_runs.into_serde().unwrap();
        let person = Person {
            runs,
        };
        JsValue::from_serde(&person).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct RelayRace {
    distance: u32,
    duration: u32,
    runs: Vec<Run>,
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
}

#[wasm_bindgen]
pub fn calculate(val: &JsValue, distance: u32) -> JsValue {
    utils::set_panic_hook();
    let persons: Vec<Person> = val.into_serde().unwrap();
    let mut fastestRelayRace: RelayRace;

    log!("len: {}", persons.len());
    log!("distance: {}", distance);

    JsValue::from_serde(&fastestRelayRace).unwrap()
}

fn addRun(remainingPersons: Vec<Person>, currentRuns: Vec<Run>, distance: u32) {
    if remainingPersons.is_empty() {
        let currentRelayRace: RelayRace = calculateRelayRace(currentRuns);
        if (currentRelayRace.distance == distance && (fastestRelayRace || fastestRelayRace.duration > currentRelayRace.duration)) {
            fastestRelayRace = currentRelayRace;
        }
        let me = remainingPersons.pop();
        if me.is_some() {
            for currentRun in me.unwrap().runs {
                let withThisRun = currentRuns.to_vec().push(currentRun);

                addRun(remainingPersons, withThisRun, distance);
            }
        }
    }
}

fn calculateRelayRace(runs: Vec<Run>) -> RelayRace {
    let mut distance: u32 = 0;
    let mut duration: u32 = 0;
    for run in runs {
        distance += run.distance;
        duration += run.duration;
    }
    RelayRace {
        distance,
        duration,
        runs
    }
}

/*
gesamtZeit(restPersonen: Person[], currentRuns: Run[], distance: int) {
    if (restPersonen.length === 0) {
        relayRace = runRelayRace(andereLäufe);
        if (relayRace.distance === distance && isFastestLauf) {
            optimalerLauf = lauf;
        }
    }

    me = restpersonen.pop();
    for (run in me.runs) {
        gesamtZeit(restpersonen, [ ...andereLäufe, run ], distance);
    }
}
*/