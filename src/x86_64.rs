use alloc::format;
use core::arch::asm;

pub fn console_print(buffer: &str) {
    let buffer_ptr = buffer.as_ptr();
    unsafe {
        asm!(
            // calculate the length of string
            "mov rdi, {buffer}",    // buffer to destination index
            "xor rcx, rcx",         // zero rcx
            "not rcx",              // set rcx = -1
            "xor al, al",           // zero the al register (initialize to NUL)
            "cld",                  // clear the direction flag
            "repnz scasb",          // get the string length (dec rcx through NUL)
            "not rcx",              // rev all bits of negative results in absolute value
            "dec rcx",              // -1 to skip the null-terminator, rcx contains length
            "mov rdx, rcx",         // put length in rdx

            // write string to stdout
            "mov rsi, {buffer}",    // buffer to source index
            "mov rax, 1",           // set write to command
            "mov rdi, rax",         // set destination index to rax (stdout)
            "syscall",              // call kernel

            buffer = in(reg) buffer_ptr,
        );
    }
}

pub fn console_print_with_nl(buffer: &str) {
    console_print(&format!("{}\n", buffer))
}
