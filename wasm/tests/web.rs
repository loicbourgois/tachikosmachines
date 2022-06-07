//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate tachikosmachines;
use tachikosmachines::Universe;
use tachikosmachines::cell_id;
use tachikosmachines::cell_ids_9;
use tachikosmachines::cell_ids_9_3;
use tachikosmachines::maths::Vector;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


#[wasm_bindgen_test]
pub fn test_cell_id() {
    let width = 10;
    assert!(cell_id(Vector {
        x: 0.0,
        y: 0.0,
    }, width) == 0);
    assert!(cell_id(Vector {
        x: 0.2,
        y: 0.0,
    }, width) == 2);
    assert!(cell_id(Vector {
        x: 0.31,
        y: 0.56,
    }, width) == 53);
}


#[wasm_bindgen_test]
pub fn test_cell_ids_9() {
    let width = 10;
    assert!(cell_ids_9(Vector {
        x: 0.0,
        y: 0.0,
    }, width) == [0,1,10,11]);
    assert!(cell_ids_9_3(59, width) == [48, 49, 58, 59, 68, 69] );
    assert!(cell_ids_9_3(99, width) == [88,89, 98,99] );
}


#[wasm_bindgen_test]
pub fn test_universe() {
    Universe::new(0.01).test();
}
