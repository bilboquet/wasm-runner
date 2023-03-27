mod rust_wasm {
    use crate::init_runtime;

    #[test]
    fn test_use_function_from_wasm() {
        let wasm_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wasm-module/tmp/wasm_module.wasm"
        ));

        let (mut store, wasm_mod) = init_runtime(wasm_bytes);

        let res = wasm_mod.add(&mut store, 1, 1).unwrap();
        assert_eq!(res, 2);

        let res = wasm_mod.add(&mut store, 2, 2).unwrap();
        assert_eq!(res, 4);
    }
    #[test]
    fn test_use_fn_from_wasm_that_use_native_host_fn() {
        let wasm_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wasm-module/tmp/wasm_module.wasm"
        ));

        let (mut store, wasm_mod) = init_runtime(wasm_bytes);
        let res = wasm_mod.start(&mut store).unwrap();
        assert_eq!(res, 0);
    }
}
