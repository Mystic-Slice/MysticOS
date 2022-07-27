
// Important to disable std library
// becoz OS cannot use anything that
// an OS provides usually (like stdout, files, etc...)
#![no_std]
// To not use the normal entry point
#![no_main]

static HELLO: &[u8] = b"Hello World!";

// Define own start function (the entry point)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Have to define a handler
// for when Rust panics
use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Have to let it run in an infinite loop
    // No other way
    loop {}
}
