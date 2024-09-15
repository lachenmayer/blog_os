#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello from main")
        .unwrap();
    write!(vga_buffer::WRITER.lock(), " - 1 / 3 = {}", 1.0 / 3.0).unwrap();
    loop {}
}
