// what is used from wasm modules
wai_bindgen_wasmer::import!("../wasm-module/tmp/wasm-mod-exported.wai");

// what this module must provide
wai_bindgen_wasmer::export!("../wasm-module/tmp/host-exported.wai");

#[derive(Default)]
struct HostExported;

impl host_exported::HostExported for HostExported {
    fn echo(&mut self) -> () {
        println!("Hello from wasm-module!");
    }
}

#[cfg(test)]
mod tests {
    use wai_bindgen_wasmer::wasmer::{imports, Module, Store};

    use crate::{wasm_mod_exported::WasmModExported};

    #[test]
    fn test_use_function_from_wasm() {
        use super::*;

        let wasm_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wasm-module/tmp/wasm_module.wasm"
        ));

        let mut store = Store::default();
        let module = Module::new(&store, &wasm_bytes).unwrap();
        // The module doesn't import anything, so we create an empty import object.
        let mut import_object = imports! {};

        // host env/context ??
        let data= HostExported::default();
        // provide native functions to wasm module
        let f = host_exported::add_to_imports(&mut store, &mut import_object, data);

        let (wasm_mod, _instance) =
            WasmModExported::instantiate(&mut store, &module, &mut import_object).unwrap();

        let res = wasm_mod.add(&mut store, 1, 1).unwrap();
        assert_eq!(res, 2);

        let res = wasm_mod.add(&mut store, 2, 2).unwrap();
        assert_eq!(res, 4);
    }

    #[test]
    fn test_use_fn_from_wasm_that_use_native_host_fn() {
        use super::*;

        let wasm_bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../wasm-module/tmp/wasm_module.wasm"
        ));

        let mut store = Store::default();
        let module = Module::new(&store, &wasm_bytes).unwrap();
        // The module doesn't import anything, so we create an empty import object.
        let mut import_object = imports! {};

        // host env/context ??
        let data= HostExported::default();
        // provide native functions to wasm module
        let f = host_exported::add_to_imports(&mut store, &mut import_object, data);

        let (wasm_mod, _instance) =
        WasmModExported::instantiate(&mut store, &module, &mut import_object).unwrap();
        // seems to be required but have no effect ???
        // f(& _instance, &store).unwrap();

        let res = wasm_mod.start(&mut store).unwrap();
        assert_eq!(res, 0);
    }
}
