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
use MysticOS::{ println };
use bootloader::{ BootInfo, entry_point };
use MysticOS::task::{ Task, executor::Executor, keyboard };

extern crate alloc;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use MysticOS::allocator;
    use MysticOS::memory::{ self, BootInfoFrameAllocator };
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    MysticOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();
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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}