// what this module must provide
wai_bindgen_wasmer::export!("../wasm-module/tmp/env.wai");

#[derive(Default)]
pub struct EnvData;

// Implementation block for "services" provided by the host
impl env::Env for EnvData {
    fn zero_arg(&mut self) {
        println!("zero_arg called!");
    }

    fn one_arg(&mut self, arg: u32) {
        println!("one_arg called with arg: {}", arg);
    }

    fn two_arg(&mut self, arg1: u32, arg2: u32) {
        println!("two_arg called! arg1: {} arg2: {}", arg1, arg2);
    }

    fn one_string_arg(&mut self, msg: &str) {
        println!("one_string_arg called with: \"{}\"!", msg);
    }

    fn abort(&mut self, msg: &str, a: u32, b: u32) {
        println!("Abort called with: \"{} {} {}\"!", msg, a, b);
    }
}
