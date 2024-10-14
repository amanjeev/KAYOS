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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // let ptr = 0x2041dc as *mut u8;
    // unsafe { let x = *ptr; }
    // println!("read worked");
    //
    // unsafe { *ptr = 42; }
    // println!("write worked");

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    kayos::hlt_loop();
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
