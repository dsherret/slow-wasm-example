
extern crate wasmer;

use wasmer::{Store, Module, Instance, imports};

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    println!("Starting...");
    let start = {
        let mut threads = Vec::new();
        let start = std::time::Instant::now();
        for _ in 0..10 {
            threads.push(std::thread::spawn(|| {
                // I haven't figure out how to create a module once and share it amongst the threads in wasmer 3
                let import_object = imports! {};
                let mut store = Store::default();
                let module = Module::new(&store, &WASM).unwrap();
                let instance = Instance::new(&mut store, &module, &import_object).unwrap();
                let format = instance.exports.get_function("format").unwrap();
                let result = format.call(&mut store, &[]).unwrap();
                (store, module, instance, result)
            }));
        }
        let mut data = Vec::new();
        for thread in threads {
            data.push(thread.join().unwrap());
        }
        println!("Finished in {}ms...", start.elapsed().as_millis());
        eprintln!("Dropping...");
        std::time::Instant::now()
    };
    println!("Finished dropping in {}ms...", start.elapsed().as_millis());
}
