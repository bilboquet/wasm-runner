use crate::init_runtime;

fn test_add(wasm: &[u8]) {
    let (mut store, wasm_mod) = init_runtime(wasm);

    let res = wasm_mod.add(&mut store, 1, 1).unwrap();
    assert_eq!(res, 2);

    let res = wasm_mod.add(&mut store, 2, 2).unwrap();
    assert_eq!(res, 4);
}

fn test_start(wasm: &[u8]) {
    let (mut store, wasm_mod) = init_runtime(wasm);
    let res = wasm_mod.start(&mut store).unwrap();
    assert_eq!(res, 0);
}

fn test_greet(wasm: &[u8]) {
    let (mut store, wasm_mod) = init_runtime(wasm);

    let res = wasm_mod.greet(&mut store, "Paul").unwrap();

    assert_eq!(res, "Hello Paul!".to_string());
}

#[test]
fn test_add_from_rust_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module/tmp/wasm_module.wasm"
    ));

    test_add(wasm_bytes);
}

#[test]
fn test_add_from_as_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module-as/wasm-module.wasm"
    ));

    test_add(wasm_bytes);
}

#[test]
fn test_start_from_rust_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module/tmp/wasm_module.wasm"
    ));

    test_start(wasm_bytes);
}

#[test]
fn test_start_from_as_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module-as/wasm-module.wasm"
    ));

    test_start(wasm_bytes);
}

#[test]
fn test_greet_from_rust_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module/tmp/wasm_module.wasm"
    ));

    test_greet(wasm_bytes);
}

#[test]
fn test_greet_from_as_wasm() {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../wasm-module-as/wasm-module.wasm"
    ));

    test_greet(wasm_bytes);
}
