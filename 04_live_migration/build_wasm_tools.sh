# From GitHub.
git clone https://github.com/TintoEdoardo/wasm-tools/tree/main

# Build the repo.
cd wasm-tools/
cargo build --release

# Extract wasm-tools.
cp wasm-tools/target/release/wasm-tools out/wasm-tools