#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
