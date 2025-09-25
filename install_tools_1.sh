# Remove previous versions.
sudo apt remove rustc -y
sudo apt autoremove -y

# Install a Rust toolchains manager.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update