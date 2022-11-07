#![no_std]
#![feature(decl_macro, alloc_error_handler)]

extern crate alloc;

mod macros;

pub use macros::*;

pub mod libc;

/// Clear the terminal window.
pub fn clear() {
    print!("\x1B[2J")
}

pub fn beep() {
    print!("\x07")
}
