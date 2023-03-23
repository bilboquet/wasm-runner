// what is used from wasm modules
wai_bindgen_wasmer::import!("../wasm-module/wasm-mod-exported.wai");

#[cfg(test)]
mod tests {
    use wai_bindgen_wasmer::wasmer::{imports, Module, Store};

    use crate::wasm_mod_exported::WasmModExported;

    #[test]
    fn test_use_function_from_wasm() {
        let wasm_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wasm-module/tmp/wasm_module.wasm"
        ));

        let mut store = Store::default();
        let module = Module::new(&store, &wasm_bytes).unwrap();
        // The module doesn't import anything, so we create an empty import object.
        let mut import_object = imports! {};

        let (wasm_mod, _instance) =
            WasmModExported::instantiate(&mut store, &module, &mut import_object).unwrap();

        let res = wasm_mod.add(&mut store, 1, 1).unwrap();
        assert_eq!(res, 2);

        let res = wasm_mod.add(&mut store, 2, 2).unwrap();
        assert_eq!(res, 4);
    }
}
