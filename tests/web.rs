//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_test::rust::rust_only;
use wasm_test::{add, upper};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(1, 1), 2);
}

#[wasm_bindgen_test]
fn test_upper() {
    assert_eq!(
        upper("something lowercase"),
        "SOMETHING LOWERCASE".to_string()
    );
}

#[wasm_bindgen_test]
fn test_rust_only() {
    assert_eq!(rust_only("ALL UPPERCASE"), "all uppercase".to_string());
}
