// what this module must provide
wai_bindgen_wasmer::export!("../wasm-module/tmp/host-exported.wai");

#[derive(Default)]
pub struct HostExportedData;

// Implementation block for "services" provided by the host
impl host_exported::HostExported for HostExportedData {
    fn hello(&mut self) {
        println!("Hello from wasm-module!");
    }
}
