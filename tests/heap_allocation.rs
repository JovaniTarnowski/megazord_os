#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(megazord_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use megazord_os::allocator::HEAP_SIZE;

extern crate alloc;

#[test_case]
fn many_boxes() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    megazord_os::test_panic_handler(info)
}

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use megazord_os::allocator;
    use megazord_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    megazord_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}
