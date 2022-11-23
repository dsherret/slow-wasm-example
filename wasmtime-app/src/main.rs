use wasmtime::*;

// run build.sh in root directory to create this
static WASM: &'static [u8] = include_bytes!("../../formatter/target/wasm32-unknown-unknown/release/formatter.wasm");

fn main() {
    let start = {
        let mut threads = Vec::new();
        let start = std::time::Instant::now();
        let engine = Engine::default();
        let module = Module::new(&engine, WASM).unwrap();
        for _ in 0..10 {
            let engine = engine.clone();
            let module = module.clone();
            threads.push(std::thread::spawn(move || {
                let mut store = Store::new(&engine, 4);
                let instance = Instance::new(&mut store, &module, &[]).unwrap();
                let format = instance.get_typed_func::<(), u32, _>(&mut store, "format").unwrap();
                let result = format.call(&mut store, ()).unwrap();
                (engine, store, module, instance, result)
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
