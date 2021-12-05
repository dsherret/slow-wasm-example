On a Windows machine (works fine on Linux and Mac):

1. Build the .wasm file by running `./build.sh`.
3. `cd native-app && cargo run --release` - Note the time it takes.
2. `cd wasmer-app && cargo run --release` - Note the time it takes.
2. `cd wasmtime-app && cargo run --release` - Note the time it takes.
4. Run a web server in the root directory (ex. `python -m SimpleHTTPServer 7800`).
5. Go to `http://localhost:7800/test.html` and open the console. Note the time it takes in different browsers.

Results on my machine:

1. Native - ~433ms
2. Wasmer (cranelift) - ~~145s~~ ~448ms -- Fixed :)
3. Wasmtime - ~436ms
3. Chrome - 5s
4. Firefox - 705ms
