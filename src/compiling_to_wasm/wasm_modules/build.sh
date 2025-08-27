cargo build --target=wasm32-unknown-unknown --release
rm -f out/gcd.wasm
mv target/wasm32-unknown-unknown/release/wasm_modules.wasm out/gcd.wasm