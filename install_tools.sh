# Remove previous versions.
sudo apt remove rustc -y
sudo apt autoremove -y

# Install a Rust toolchains manager.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Get some toolchains we might use today.
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasip1
rustup target add wasm32-wasip2

# Get wasm-tools.
