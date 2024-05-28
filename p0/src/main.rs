#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
fn _start() {
    loop{

    }
}

// 使用panic! 宏时，调用on_panic
#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    loop {

    }
}