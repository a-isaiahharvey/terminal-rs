#![no_std]
#![feature(decl_macro, alloc_error_handler)]

extern crate alloc;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

mod macros;
pub use macros::*;

mod allocator;

/// Clear the terminal window.
pub fn clear() {
    print!("\x1B[2J")
}

pub fn beep() {
    print!("\x07")
}
