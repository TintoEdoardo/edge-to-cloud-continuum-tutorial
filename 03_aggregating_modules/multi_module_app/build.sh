# Cleaning of previous run.
rm -r out/ &> /dev/null
mkdir out/ &> /dev/null

# Copy the Wasm module to `out/`.
cp wasm_modules/module_1.wat out/module_1.wat
cp wasm_modules/module_2.wat out/module_2.wat

# Build the Rust host component.
cargo build

# Place relevant files into out.
cp target/debug/multi_module_app out/main
