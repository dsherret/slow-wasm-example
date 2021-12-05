use wasmtime::*;

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    let engine = Engine::default();
    let module = Module::new(&engine, WASM).unwrap();
    let mut store = Store::new(&engine, 4);
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let format = instance.get_typed_func::<(), u32, _>(&mut store, "format").unwrap();

    let start = std::time::Instant::now();
    let result = format.call(&mut store, ()).unwrap();
    println!("Finished in {}ms...", start.elapsed().as_millis());
    println!("Result: {:?}", result);
}
