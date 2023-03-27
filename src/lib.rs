use crate::host::{host_exported, HostExportedData};
use wai_bindgen_wasmer::wasmer::{imports, Module, Store};
use wasm_mod_exported::WasmModExported;

// what is used from wasm modules
wai_bindgen_wasmer::import!("../wasm-module/tmp/wasm-mod-exported.wai");

pub mod host;

pub fn init_runtime(wasm_bytes: &[u8]) -> (Store, WasmModExported) {
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    // We create an empty import object. Imports are provided later `see add_to_imports`
    let mut import_object = imports! {};

    // host env/context ??
    let data = HostExportedData::default();
    // provide native functions to wasm module
    let _f = host_exported::add_to_imports(&mut store, &mut import_object, data);

    let (wasm_mod, _instance) =
        WasmModExported::instantiate(&mut store, &module, &mut import_object).unwrap();
    (store, wasm_mod)
}

#[cfg(test)]
mod tests;
