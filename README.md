1. Build the .wasm file by running `./build.sh`.
1. Run each of these commands a few time to get an average.
   1. `cd wasmer2-app && cargo run --release`
   1. `cd wasmer3-app && cargo run --release`
   1. `cd wasmtime-app && cargo run --release`

Results on my Windows 10 machine - i9-10900K - 20 logical cores (huge approximations):

1. Wasmer 2.3
   - Formatting time: ~4800ms
   - Drop time: ~720ms
1. Wasmer 3.0
   - Formatting time: ~6500ms -- slow because Module needs to be created per thread
   - Drop time: **~3000ms**
1. Wasmtime 3.0
   - Formatting time: ~4800ms
   - Drop time: ~840ms

This may just be a question about the use of Wasmer 3.0's API. It seems I now have to create a `Module` and `Store` per thread, which is leading to a huge slowdown when dropping them all.

In the examples, everything is dropped on the main thread to mimic the "plugin pool" that dprint creates.