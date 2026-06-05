//! Credit to https://surma.dev/things/rust-to-webassembly/.

/*** STEP 1: use no_mangle *** */

/*
#![no_std]
*/

#[unsafe(no_mangle)]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
*/