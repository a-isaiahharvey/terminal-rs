#![no_std]
#![no_main]
#![feature(lang_items)]

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

use core::arch::asm;

use terminal_rs::println;

// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    println!("Hi macOS!");
    0
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
