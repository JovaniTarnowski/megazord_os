#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(megazord_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use megazord_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    megazord_os::test_panic_handler(info)
}

/// By testing println in a basic_boot environment without calling any initialization routines in _start,
/// we can ensure that println works right after booting.
/// This is important because we rely on it e.g. for printing panic messages.
#[test_case]
fn test_println() {
    println!("test_println output");
}
