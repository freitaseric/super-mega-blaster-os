#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Error");
    loop {}
}
