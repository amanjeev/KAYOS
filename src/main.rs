#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kayos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kayos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    kayos::init();


    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();


    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kayos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}