
extern crate wasmer;

use wasmer::{Store, Module, Instance, imports};

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    println!("Starting...");
    let import_object = imports! {};
    let store = Store::default();
    let module = Module::new(&store, &WASM).unwrap();
    let instance = Instance::new(&module, &import_object).unwrap();
    let format = instance.exports.get_function("format").unwrap();

    let start = std::time::Instant::now();
    let result = format.call(&[]).unwrap();
    println!("Finished in {}ms...", start.elapsed().as_millis());
    println!("Result: {:?}", result[0]);
}
