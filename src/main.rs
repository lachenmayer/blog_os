#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"da best os";

#[no_mangle]
extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        let offset = i as isize * 2;
        unsafe {
            *vga_buffer.offset(offset) = byte;
            *vga_buffer.offset(offset + 1) = 0xb;
        }
    }

    loop {}
}
