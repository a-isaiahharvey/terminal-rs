use core::arch::asm;

use alloc::format;

pub fn console_print(buffer: &str) {
    let buffer_ptr = buffer.as_ptr();
    unsafe {
        asm!(
            "mov X0, #1",           // 1 = StdOut
            "mov X1, {buffer}",     // string to print
            "mov x2, #13",          // length of our string
            "mov x16, #4",          // Unix write system call
            "svc #0x80",            // Call kernel to output the string
            buffer = in(reg) buffer_ptr,
        );
    }
}

pub fn console_print_with_nl(buffer: &str) {
    console_print(&format!("{}\n", buffer))
}
