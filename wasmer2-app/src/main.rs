
extern crate wasmer;

use wasmer::{Store, Module, Instance, imports};

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    println!("Starting...");
    let start = {
        let mut threads = Vec::new();
        let start = std::time::Instant::now();
        // this api was nice because I could create the module once
        let store = Store::default();
        let module = Module::new(&store, &WASM).unwrap();
        for _ in 0..10 {
            let module = module.clone();
            threads.push(std::thread::spawn(move || {
                let import_object = imports! {};
                let instance = Instance::new(&module, &import_object).unwrap();
                let format = instance.exports.get_function("format").unwrap();
                let result = format!("{:?}", format.call(&[]).unwrap()[0]);
                (instance, result)
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
