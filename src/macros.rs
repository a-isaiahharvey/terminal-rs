pub macro print {
    () => {
        $crate::console_print("")
    },

    ($fmt:literal) => {
        $crate::console_print($fmt)
    },

    ($fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            let fmt_string = alloc::format!($fmt, $($arg)*);
            $crate::console_print(&fmt_string)
        }
    },
}

pub macro println {
    () => {
        $crate::console_print("\n")
    },

    ($fmt:literal) => {
        $crate::console_print_with_nl($fmt)
    },

    ($fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            let fmt_string = alloc::format!($fmt, $($arg)*);
            $crate::console_print_with_nl(&fmt_string)
        }
    },
}
