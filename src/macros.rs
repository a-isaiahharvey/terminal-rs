pub macro print {
    () => {
        unsafe {
            $crate::libc::printf("");
        }
    },

    ($fmt:literal) => {
        unsafe {
            $crate::libc::printf($fmt.as_ptr() as *const i8);
        }
    },

    ($fmt:expr, $($arg:tt)*) => {
        unsafe {
            $crate::libc::printf(&fmt.as_ptr() as *const i8)
        }
    },
}

pub macro println {
    () => {
        unsafe {
            libc::printf("\n");
        }
    },

    ($fmt:literal) => {
        unsafe {
            extern crate alloc;
            let with_nl = alloc::format!("{}\n", $fmt);
            $crate::libc::printf(with_nl.as_ptr() as *const i8);
        }
    },

    ($fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            let mut fmt_string = alloc::format!($fmt, $($arg)*);
            fmt_string = alloc::format!("{}\n", fmt_string);
            $crate::libc::printf(fmt_string.as_ptr() as *const i8);
        }
    },
}
