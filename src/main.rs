#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("don't panic: {}", info);
    loop {}
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!(" 1 / 3 = {}", 1.0 / 3.0);
    loop {}
}
