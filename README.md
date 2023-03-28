# Wasm experiment: use a wasm module

## Goals

### 1

`test_use_function_from_wasm()`

Simply use a function from https://github.com/bilboquet/wasm-module commit #7dbf1c0

### 2

`test_use_fn_from_wasm_that_use_native_host_fn`
Use an update version of https://github.com/bilboquet/wasm-module commit #4173e1d
The updated wasm-module calls a function provided by the host

### 3

add new functions and tests
also test a module written in AS: see https://github.com/bilboquet/wasm-module-as.git
