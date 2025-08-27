//! Credit to https://surma.dev/things/rust-to-webassembly/

#[unsafe(no_mangle)]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}

