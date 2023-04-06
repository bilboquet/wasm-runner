use crate::{wasm_as, wasm_rust};

#[cfg(feature = "rust")]
#[test]
fn test_add_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.add(&mut store, 1, 1).unwrap();
    assert_eq!(res, 2);

    let res = wasm_mod.add(&mut store, 2, 2).unwrap();
    assert_eq!(res, 4);
}

#[cfg(feature = "AS")]
#[test]
fn test_add_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.add(&mut store, 1, 1).unwrap();
    assert_eq!(res, 2);

    let res = wasm_mod.add(&mut store, 2, 2).unwrap();
    assert_eq!(res, 4);
}

#[cfg(feature = "rust")]
#[test]
fn test_call_hello_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.call_hello(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "AS")]
#[test]
fn test_call_hello_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.call_hello(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "rust")]
#[test]
fn test_call_one_array_arg_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.call_one_array_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "AS")]
#[test]
fn test_call_one_array_arg_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.call_one_array_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "rust")]
#[test]
fn test_greet_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.greet(&mut store, "Paul").unwrap();

    assert_eq!(res, "Hello Paul!".to_string());
}

#[cfg(feature = "AS")]
#[test]
fn test_greet_as() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.greet(&mut store, "Paul").unwrap();

    println!("{}", res);

    assert_eq!(res, "Hello Paul!".to_string());
}

#[cfg(feature = "rust")]
#[test]
fn test_fillarray_u8_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let len = 7;
    let value = 42;
    let res = wasm_mod.fillarray_u8(&mut store, len, value);

    let res = res.unwrap();

    assert_eq!(res.len(), len as usize);
    assert_eq!(res, vec![value; len as usize]);
}

#[cfg(feature = "AS")]
#[test]
fn test_fillarray_u8_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let len = 7;
    let value = 42;
    let res = wasm_mod.fillarray_u8(&mut store, len, value);

    let res = res.unwrap();

    assert_eq!(res.len(), len as usize);
    assert_eq!(res, vec![value; len as usize]);
}

#[cfg(feature = "rust")]
#[test]
fn test_fillarray_static_u8_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let vector = vec![1, 2, 3, 4, 5];
    let res = wasm_mod.fillarray_static_u8(&mut store, &vector).unwrap();

    assert_eq!(res.len(), vector.len());
    assert_eq!(res, vec![2, 3, 4, 5, 6]);
}

#[cfg(feature = "AS")]
#[test]
fn test_fillarray_static_u8_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let vector = vec![1, 2, 3, 4, 5];
    let res = wasm_mod.fillarray_static_u8(&mut store, &vector).unwrap();

    assert_eq!(res.len(), vector.len());
    assert_eq!(res, vec![2, 3, 4, 5, 6]);
}

#[cfg(feature = "rust")]
#[test]
fn test_call_zero_arg_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.call_zero_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "AS")]
#[test]
fn test_call_zero_arg_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.call_zero_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "rust")]
#[test]
fn test_call_one_arg_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.call_one_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "AS")]
#[test]
fn test_call_one_arg_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.call_one_arg(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "rust")]
#[test]
fn test_call_two_args_rust() {
    let (mut store, wasm_mod) = wasm_rust::init_runtime();

    let res = wasm_mod.call_two_args(&mut store).unwrap();
    assert_eq!(res, 0);
}

#[cfg(feature = "AS")]
#[test]
fn test_call_two_args_as() {
    let (mut store, wasm_mod) = wasm_as::init_runtime();

    let res = wasm_mod.call_two_args(&mut store).unwrap();
    assert_eq!(res, 0);
}
