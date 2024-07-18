#![no_std]
#![no_main]
use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(vga_buffer::WRITER.lock(), "{} - {}\n", 1, 1.5).unwrap();
    write!(vga_buffer::WRITER.lock(), "random text").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
