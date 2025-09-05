//! Credit to https://surma.dev/things/rust-to-webassembly/.

/*** STEP 2: import a function ***
#[link(wasm_import_module = "Math")]
unsafe extern "C" {
    fn clock() -> u64;
} */

/*** STEP 1: use no_mangle ***
 #[unsafe(no_mangle)] */
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}

