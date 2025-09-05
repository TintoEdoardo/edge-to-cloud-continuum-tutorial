# Cleaning of previous run.
rm -r out/ &> /dev/null
mkdir out/ &> /dev/null

# Actual build (in release mode).
cd wasm_modules
cargo build --release --target wasm32-wasip1
cargo build --release --target wasm32-unknown-unknown
cd ..

# Build the Rust host component.
cd host_component
cargo build
cd ..

# Place relevant files into out.
cp wasm_modules/target/wasm32-unknown-unknown/release/wasm_modules.wasm out/module_unknown.wasm
cp wasm_modules/target/wasm32-wasip1/release/wasm_modules.wasm out/module_wasip1.wasm
cp host_component/target/debug/host_component out/main

# Print both wasm module into textual form.
cd out
wasm-tools print module_unknown.wasm > module_unknown.wat
wasm-tools print module_wasip1.wasm > module_wasip1.wat