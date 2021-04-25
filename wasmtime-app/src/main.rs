use wasmtime::*;

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    let store = Store::default();
    let module = Module::new(store.engine(), WASM).unwrap();
    let instance = Instance::new(&store, &module, &[]).unwrap();
    let format = instance.get_typed_func::<(), u32>("format").unwrap();

    let start = std::time::Instant::now();
    let result = format.call(()).unwrap();
    println!("Finished in {}ms...", start.elapsed().as_millis());
    println!("Result: {:?}", result);
}
