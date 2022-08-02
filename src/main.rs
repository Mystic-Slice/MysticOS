// Important to disable std library
// becoz OS cannot use anything that
// an OS provides usually (like stdout, files, etc...)
#![no_std]
// To not use the normal entry point
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(MysticOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use MysticOS::println;

// Define own start function (the entry point)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    MysticOS::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    MysticOS::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    MysticOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    MysticOS::test_panic_handler(info)
}
