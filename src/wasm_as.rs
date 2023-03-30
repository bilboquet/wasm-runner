use crate::{
    env::{self, EnvData},
    host::{host_exported, HostExportedData},
};

use wai_bindgen_wasmer::wasmer::{imports, Module, Store};

// What is used from wasm modules
// auto generate bindings at build time
// /!\ this means that one MUST know the Interface at build time
wai_bindgen_wasmer::import!("../../wit-mem-transform/as_sc/build/wasm-interface.wai");

type WasmModule = wasm_interface::WasmInterface;

pub fn init_runtime() -> (Store, WasmModule) {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../wit-mem-transform/as_sc/build/jf.wasm"
    ));

    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes).unwrap();
    // We create an empty import object. Imports are provided later `see add_to_imports`
    let mut import_object = imports! {};

    // host env/context ??
    let host_data = HostExportedData::default();
    // provide native functions to wasm module
    let apply_config_for_host =
        host_exported::add_to_imports(&mut store, &mut import_object, host_data);

    let env_data = EnvData::default();
    // provide native functions to wasm module
    let apply_config_for_env = env::env::add_to_imports(&mut store, &mut import_object, env_data);

    let (wasm_mod, instance) =
        WasmModule::instantiate(&mut store, &module, &mut import_object).unwrap();

    // MUST be called, at least for the memory to be shared
    apply_config_for_host(&instance, &store).unwrap();
    apply_config_for_env(&instance, &store).unwrap();

    (store, wasm_mod)
}
