use crate::wasm_rust;

#[test]
fn test_add_from_rust_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.add(&mut store, 1, 1).unwrap();
    assert_eq!(res, 2);

    let res = wasm_mod.add(&mut store, 2, 2).unwrap();
    assert_eq!(res, 4);
}

#[test]
fn test_add_from_as_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.add(&mut store, 1, 1).unwrap();
    assert_eq!(res, 2);

    let res = wasm_mod.add(&mut store, 2, 2).unwrap();
    assert_eq!(res, 4);
}

#[test]
fn test_start_from_rust_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.start(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[test]
fn test_start_from_as_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();
    
    let res = wasm_mod.start(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[test]
fn test_greet_from_rust_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.greet(&mut store, "Paul").unwrap();

    assert_eq!(res, "Hello Paul!".to_string());
}

#[test]
fn test_greet_from_as_wasm() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.greet(&mut store, "Paul").unwrap();

    assert_eq!(res, "Hello Paul!".to_string());
}
