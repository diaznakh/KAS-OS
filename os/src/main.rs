//main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

//function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//overwriting the entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

fn main(){

}