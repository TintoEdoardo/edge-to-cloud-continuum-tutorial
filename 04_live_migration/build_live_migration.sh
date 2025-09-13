# Build the repo.
cd live_migration
cargo build
cd ..

# Extract wasm-tools.
cp live_migration/target/debug/live_migration out/main