# Cleaning of previous run.
rm -r out/ &> /dev/null
mkdir out/ &> /dev/null

# Build the gcd application.
cd src/gcd
cargo build
cd ..

# Build the memory application.
cd memory
cargo build
cd ../..

# Place relevant files into out.
cp src/gcd/target/debug/gcd out/gcd
cp src/memory/target/debug/memory out/memory
cp wasm_modules/gcd.wat out/gcd.wat
cp wasm_modules/memory.wat out/memory.wat
